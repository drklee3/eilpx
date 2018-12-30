pub mod error;
mod config;
mod util;
use crate::error::Result;
use crate::util::setup_logger;
use crate::config::Config;

use clap::{Arg, App};
use image::{DynamicImage, GenericImageView, ImageBuffer};
use log::{error, debug, info};
use std::{path::Path, process};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("input")
            .help("Sets the input file")
            .short("i")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("direction")
            .help("Sets direction of sorting")
            .short("d")
            .default_value("horizontal")
            .possible_values(&["horizontal", "vertical"])
            .case_insensitive(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("mode")
            .help("Sets mode of sorting")
            .short("m")
            .default_value("luma")
            .possible_values(&["red", "green", "blue", "alpha", "luma"])
            .case_insensitive(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("threshold")
            .help("Sets threshold of sorting")
            .short("t")
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .help("Sets the output file")
            .short("o")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("verbosity")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();
    
    // get file input
    let input = matches
        .value_of("input")
        .unwrap();

    let verbosity: u64 = matches.occurrences_of("verbosity");
    setup_logger(verbosity)?;

    info!("Opening file: {}", input);

    let img = match image::open(input) {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to open file: {}", e);
            process::exit(1);
        }
    };

    let direction = matches
        .value_of("direction")
        .unwrap();
    
    let mode = matches
        .value_of("mode")
        .unwrap();
    
    let threshold = matches
        .value_of("threshold");

    let conf = Config::new(direction, mode, threshold);

    debug!("Using configuration: {:?}", conf);
    
    let (width, height) = img.dimensions();
    info!("Sorting {}x{} image", width, height);

    // sort image duh
    let sorted = sort_image(img, conf);

    let output_path = matches
        .value_of("output")
        .map(Path::new)
        .unwrap();
    // save image duh
    save_image(sorted, output_path);

    Ok(())
}

fn get_value(config: &Config, pixel: &image::Rgba<u8>) -> u8 {
    match config.mode {
        config::Mode::Red   => pixel.data[2],
        config::Mode::Green => pixel.data[1],
        config::Mode::Blue  => pixel.data[0],
        config::Mode::Alpha => pixel.data[3],
        config::Mode::Luma  => {
            // https://en.wikipedia.org/wiki/Relative_luminance
             (0.2126 * pixel.data[2] as f64
             + 0.7152 * pixel.data[1] as f64
             + 0.0722 * pixel.data[0] as f64) as u8
        },
    }
}

fn meets_threshold(config: &Config, pixel: &image::Rgba<u8>) -> bool {
    let value = get_value(config, pixel);
    // if value is greater than the threshold,
    // could add option to let threshold be upper limit instead of lower
    value > config.threshold
}

fn sort_image(mut img: DynamicImage, config: Config)
    -> DynamicImage {
    // rotate 90 deg to do vertical sorting
    if config.direction.is_vertical() {
        img = img.rotate90();
    }

    let (width, height) = img.dimensions();

    let buf = img.to_rgba();
    // vec of pixels rgb? so cant sort directly, need to convert
    // to vec<T> where T is an entire pixel w/ RGB data 
    let mut pixels: Vec<_> = img.pixels().collect();
    let mut start: i32 = -1;
    let mut end: i32 = -1;
    let mut row_num = 0;

    for (x, y, pixel) in buf.enumerate_pixels() {
        // new row, reset
        if row_num != y {
            start = -1;
            end = - 1;
        }

        // if reaches end of a slice that meets threshold
        // OR if it reaches the end of a row
        if !meets_threshold(&config, &pixel) || x == width - 1 {
            // sort a slice if there is a start / end
            if start != -1 && end > start {
                // ok to cast to u32 since checking if start/end is positive
                let start_index = (y * width + start as u32) as usize;
                let end_index = (y * width + end as u32) as usize;

                debug!("sorting [{}..{}]", start_index, end_index);

                pixels[start_index..end_index]
                    .sort_by_key(|p| get_value(&config, &p.2));
            }

            // reset threshold
            start = -1;
        } else if start == -1 {
            // update start pos if only it is
            // the first that meets threshold
            start = x as i32;
        } else if start != -1 {
            end = x as i32;
        }

        row_num = y;
    }

    let raw_pixels: Vec<u8> = pixels
        .iter()
        .flat_map(|p| p
            .2.data.iter()
        )
        .map(|x| x.clone())
        .collect();

    // uses original width/height so should be large enough
    let buf = ImageBuffer::from_vec(width, height, raw_pixels)
        .unwrap();
    let mut img = DynamicImage::ImageRgba8(buf);

    // rotate back to original
    if config.direction.is_vertical() {
        img = img.rotate270();
    }

    img
}

fn save_image(img: DynamicImage, path: &Path) {
    let path_str = match path
        .to_str() {
            Some(path) => path,
            None => {
                error!("Invalid path characters");
                process::exit(1);
            }
        };

    match img.save(path) {
        Ok(()) => {
            info!("Saved image to: {}", path_str);
        },
        Err(e) => {
            error!("Failed to save output image: {}", e);
        }
    }
}
