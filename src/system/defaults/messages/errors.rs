use crate::system::defaults::messages::infos;
use std::process;

pub const COULD_NOT_CREATE_FILE: &'static str = "Couldn't create the file!";
pub const COULD_NOT_WRITE_FILE: &'static str = "Couldn't write the file!";
pub const HISTOGRAM_IS_EMPTY: &'static str = "Histogram is empty!";
pub const NOT_COMPUTABLE_MEDIAN: &'static str = "Not computable median!";

pub const NOT_EXISTENT_BOOLEAN: &'static str = "Not existent boolean: ";
pub const NOT_EXISTENT_CHANNEL: &'static str = "Not existent channel: ";
pub const NOT_EXISTENT_FILTER: &'static str = "Not existent filter: ";
pub const NOT_EXISTENT_PATH: &'static str = "Not existent path: ";

pub const NOT_VALID_INTEGER: &'static str = "Not valid integer: ";

pub const PARAMETER_WRONG_FORMAT: &'static str = "Wrong format of parameter: ";

pub fn print_error_and_quit(error: &'static str, error_value: Option<&str>) -> ! {
    print_error(error, error_value.unwrap_or(""));

    println!("{}", infos::QUITTED_PROGRAM);
    process::exit(1)
}

fn print_error(error: &'static str, error_value: &str) {
    eprint!("{}", error);
    eprintln!("{}", error_value);
}

