pub mod error;
mod util;
use crate::error::Result;
use crate::util::setup_logger;

use clap::{Arg, App};
use image::{DynamicImage, GenericImageView};
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
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .help("Sets the output file")
            .short("o")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();
    
    // get file input
    let input = matches
        .value_of("input")
        .unwrap();

    let verbosity: u64 = matches.occurrences_of("verbose");
    setup_logger(verbosity)?;

    info!("Opening file: {}", input);

    let mut img = match image::open(input) {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to open file: {}", e);
            process::exit(1);
        }
    };

    // sort image duh
    sort_image(&mut img);

    let output_path = matches
        .value_of("output")
        .map(Path::new)
        .unwrap();
    // save image duh
    save_image(img, output_path);

    Ok(())
}

fn sort_image(img: &mut DynamicImage, config: ) {
    let (width, height) = img.dimensions();
    info!("Sorting {}x{} image", width, height);
    let pixels = img.raw_pixels();

    for 
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
