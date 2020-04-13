use crate::system::defaults::messages::infos;
use std::process;

pub const COULD_NOT_CREATE_FILE: &'static str = "Couldn't create the file!";
pub const COULD_NOT_WRITE_FILE: &'static str = "Couldn't write the file!";
pub const HISTOGRAM_IS_EMPTY: &'static str = "Histogram is empty!";
pub const NOT_COMPUTABLE_MEDIAN: &'static str = "Not computable median!";

pub const NOT_VALID_BOOLEAN: &'static str = "Not valid boolean: ";
pub const NOT_VALID_CHANNEL: &'static str = "Not valid channel: ";
pub const NOT_VALID_FILTER: &'static str = "Not valid filter: ";
pub const NOT_VALID_NUMBER: &'static str = "Not valid number: ";
pub const NOT_VALID_NUMBER_STRING: &'static str = "Not valid number-string: ";
pub const NOT_VALID_OPERATION: &'static str = "Not valid operation: ";
pub const NOT_VALID_PATH: &'static str = "Not valid path: ";

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

