pub mod error;
mod config;
mod util;
use crate::error::Result;
use crate::util::setup_logger;
use crate::config::Config;

use clap::{Arg, App};
use image::{DynamicImage, GenericImageView, ImageBuffer};
use log::{error, debug, info, trace};
use std::{io, path::Path, process};
use std::io::Write;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("input")
            .help("Sets the input file")
            .short("i")
            .long("input")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("direction")
            .help("Sets direction of sorting")
            .short("d")
            .long("direction")
            .default_value("right")
            .possible_values(&["up", "right", "down", "left"])
            .case_insensitive(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("mode")
            .help("Sets mode of sorting")
            .short("m")
            .long("mode")
            .default_value("luma")
            .possible_values(&["red", "green", "blue", "alpha", "luma"])
            .case_insensitive(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("threshold")
            .help("Sets threshold of sorting")
            .short("t")
            .long("threshold")
            .takes_value(true)
        )
        .arg(Arg::with_name("bound")
            .help("Sets threshold to be max or min")
            .short("b")
            .long("bound")
            .default_value("min")
            .possible_values(&["min", "max"])
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .help("Sets the output file")
            .short("o")
            .long("output")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("overwrite")
            .help("Overwrite output files without asking")
            .short("y")
        )
        .arg(Arg::with_name("skip_existing")
            .help("Do not overwrite output files, exit immediately if output file already exists")
            .short("n")
            .conflicts_with("overwrite")
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
    
    let bound = matches
        .value_of("bound")
        .unwrap();
    
    let output_path_str = matches
        .value_of("output")
        .unwrap();
    
    let output_path = Path::new(output_path_str);
    
    let should_overwrite = matches
        .is_present("overwrite");
    
    let should_skip = matches
        .is_present("skip_existing");
    
    if output_path.is_file() && !should_overwrite {
        if should_skip {
            error!("File `{}` already exists.", output_path_str);
            process::exit(0);
        }

        print!("File '{}' already exists.  Overwrite? [y/N] ",
            output_path_str);
        
        // flush stdout to display line above
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        // get user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        
        // quit if input isn't y
        if input != "y\n" {
            info!("Exiting.");
            process::exit(1);
        }
    }

    let conf = Config::new(direction, mode, threshold, bound);

    debug!("Using configuration: {:?}", conf);
    
    let (width, height) = img.dimensions();
    info!("Sorting {}x{} image {}wards based on {} with {} bound of {}",
        width, height,
        direction, mode,
        conf.bound,
        conf.threshold);

    // sort image duh
    let sorted = sort_image(img, conf);

    // save image duh
    save_image(sorted, output_path);

    Ok(())
}

/// Gets the value of a pixel based on configuration mode
fn get_value(config: &Config, pixel: &image::Rgba<u8>) -> u8 {
    match config.mode {
        config::Mode::Red   => pixel.data[0],
        config::Mode::Green => pixel.data[1],
        config::Mode::Blue  => pixel.data[2],
        config::Mode::Alpha => pixel.data[3],
        config::Mode::Luma  => {
            // https://en.wikipedia.org/wiki/Relative_luminance
             (0.2126 * pixel.data[0] as f64
             + 0.7152 * pixel.data[1] as f64
             + 0.0722 * pixel.data[2] as f64) as u8
        },
    }
}

/// Returns true if a pixel meets the configured threshold
fn meets_threshold(config: &Config, pixel: &image::Rgba<u8>) -> bool {
    let value = get_value(config, pixel);

    if config.bound == "min" {
        // value is min threshold
        value > config.threshold
    } else {
        // value is max threshold
        value < config.threshold
    }
}

/// Sorts an image
fn sort_image(mut img: DynamicImage, config: Config)
    -> DynamicImage {
    // rotate 90 deg to do vertical sorting
    if config.direction.is_vertical() {
        img = img.rotate90();
    }

    // 8 bits not enough for signed
    let rev: i16 = if config.direction.is_reverse() {
        -1
    } else {
        1
    };

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

                trace!("sorting [{}..{}]", start_index, end_index);

                pixels[start_index..end_index]
                    .sort_by_key(|p| rev * get_value(&config, &p.2) as i16);
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

/// Saves a DynamicImage to disk
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
