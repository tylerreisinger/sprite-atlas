use std::path;
use std::u32;

use clap::{self, Arg};

#[derive(Default, Debug)]
pub struct InputOptions {
    pub directories: Vec<path::PathBuf>,
    pub max_width: u32,
    pub max_height: u32,
}

pub fn parse_ops() -> InputOptions {
    let opts = clap::App::new("texture-atlas")
        .version("0.1")
        .about("Generate texture atlasses from individual sprites")
        .arg(Arg::with_name("directories")
            .help("Directories to include")
            .short("D")
            .long("directories")
            .value_name("DIRECTORIES")
            .multiple(true)
            .required(true)
        )
        .arg(Arg::with_name("max_width")
            .short("w")
            .long("width")
            .value_name("MAX_WIDTH")
            .help("Maximum width of output image")
            .default_value("32000")
        )
        .arg(Arg::with_name("max_height")
            .short("h")
            .long("height")
            .value_name("MAX_HEIGHT")
            .help("Maximum height of output image")
            .default_value("32000")
        ).get_matches();

    let directories = opts.values_of("directories")
        .expect("One or more directories must be specified")
        .map(|s| path::PathBuf::from(s))
        .collect::<Vec<_>>();
    let max_width = opts.value_of("max_width")
        .unwrap()
        .parse::<u32>().expect("max_width must be a valid integer value");
    let max_height = opts.value_of("max_height")
        .unwrap()
        .parse::<u32>().expect("max_height must be a valid integer value");

    InputOptions {
        directories,
        max_width,
        max_height,
    }
}
