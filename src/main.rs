extern crate clap;
extern crate image;

use clap::{App, Arg};

use system::defaults::cli::commands;
use crate::system::argument_executor;

mod logic;
mod system;

fn main() {
    argument_executor::execute(&App::new("filter")
        .version("1.0.0")
        .author("Alexander Mattheis <alexander.mattheis@web.de>")
        .about("Applies filters to images")
        .arg(Arg::with_name(commands::INPUT)
            .short("i")
            .long(commands::INPUT)
            .takes_value(true)
            .required(true)
            .help("The path to the image(s) on which you want to apply a filter"))
        .arg(Arg::with_name(commands::REFERENCE)
            .short("r")
            .long(commands::REFERENCE)
            .takes_value(true)
            .required(false)
            .help("The path to a reference image which is necessary to compute the filter"))
        .arg(Arg::with_name(commands::FILTER)
            .short("f")
            .long(commands::FILTER)
            .takes_value(true)
            .required(true)
            .help("The filter you want to apply on the image(s)"))
        .arg(Arg::with_name(commands::PARAMS)
            .short("p")
            .long(commands::PARAMS)
            .takes_value(true)
            .required(false)
            .help("The data for the filter you want to apply"))
        .arg(Arg::with_name(commands::OUTPUT)
            .short("o")
            .long(commands::OUTPUT)
            .takes_value(true)
            .required(true)
            .help("The output-path of the filtered image(s)"))
        .get_matches());
}


