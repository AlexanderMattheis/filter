extern crate clap;
extern crate image;

use clap::{App, Arg};

use system::defaults::cli::commands;

use crate::system::argument_executor;
use crate::system::defaults::cli::manuals::ALGORITHM_MANUALS;

mod logic;
mod system;

fn main() {
    argument_executor::execute(&App::new("Filter")
        .version("0.1.0")
        .author("Alexander Mattheis <alexander.mattheis@web.de>")
        .about("Apply filters to images")
        .arg(Arg::with_name(commands::INPUT)
            .short("i")
            .long(commands::INPUT)
            .takes_value(true)
            .required(false)
            .long_help("The path to the image(s) on which you want to apply a filter"))
        .arg(Arg::with_name(commands::REFERENCE)
            .short("r")
            .long(commands::REFERENCE)
            .takes_value(true)
            .required(false)
            .long_help("The path to a reference image which is necessary to compute the filter"))
        .arg(Arg::with_name(commands::ACTION)
            .short("a")
            .long(commands::ACTION)
            .takes_value(true)
            .required(false)
            .long_help("The action you want to apply on the image(s)"))
        .arg(Arg::with_name(commands::PARAMS)
            .short("p")
            .long(commands::PARAMS)
            .takes_value(true)
            .required(false)
            .long_help("The data for the filter you want to apply"))
        .arg(Arg::with_name(commands::OUTPUT)
            .short("o")
            .long(commands::OUTPUT)
            .takes_value(true)
            .required(false)
            .long_help("The output-path of the filtered image(s)"))
        .arg(Arg::with_name(commands::MANUAL)
            .short("m")
            .long(commands::MANUAL)
            .takes_value(true)
            .required(false)
            .long_help("Prints available algorithms with its default values")
            .possible_values(&ALGORITHM_MANUALS))
        .get_matches());
}


