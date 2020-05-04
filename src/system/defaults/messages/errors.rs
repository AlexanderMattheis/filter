use std::process;

use crate::system::defaults::messages::infos;

/*
The messages here should give an answer on the question "What is the error?".
They shouldn't answer on the question "What to do to solve the error?".
*/

// failed
pub const FAILED_CREATING_FILE: &'static str = "Failed creating the file!";
pub const FAILED_LOADING_IMAGE: &'static str = "Failed loading the image: ";
pub const FAILED_LOADING_REFERENCE_IMAGE: &'static str = "Failed loading the reference image: ";
pub const FAILED_SAVING_IMAGE: &'static str = "Failed saving the image: ";
pub const FAILED_WRITING_FILE: &'static str = "Failed writing the file!";

// not existent
pub const NOT_EXISTENT_HISTOGRAM: &'static str = "Histogram is empty!";
pub const NOT_EXISTENT_MEDIAN: &'static str = "Not existent median!";
pub const NOT_EXISTENT_MANUAL: &'static str = "Not existent manual!";

// not used
pub const NOT_USED_LUMINANCE: &'static str = "Not used luminance value has been passed!";

// not same
pub const NOT_SAME_SIZE_PICTURES: &'static str = "Not same size of pictures!";

// not valid
pub const NOT_VALID_BOOLEAN: &'static str = "Not valid boolean: ";
pub const NOT_VALID_BORDER_HANDLING: &'static str = "Not valid border-handling: ";
pub const NOT_VALID_CHANNEL: &'static str = "Not valid channel: ";
pub const NOT_VALID_COLOR: &'static str = "Not valid color: ";
pub const NOT_VALID_FILTER: &'static str = "Not valid filter: ";
pub const NOT_VALID_NUMBER: &'static str = "Not valid number: ";
pub const NOT_VALID_NUMBER_STRING: &'static str = "Not valid number-string: ";
pub const NOT_VALID_OPERATION: &'static str = "Not valid operation: ";
pub const NOT_VALID_PATH: &'static str = "Not valid path: ";

// values
pub const MINIMUM_BIGGER_MAXIMUM: &'static str = "The minimum is bigger than the maximum.";

pub const QUANTILES_SUM_HIGHER_1_0: &'static str = "The sum of lower and higher quantile is higher than 1.0!";

pub const VALUE_HIGHER_1: &'static str = "The value is higher than 1.0!";
pub const VALUE_HIGHER_255: &'static str = "The value is higher than 255!";
pub const VALUE_LOWER_1_DIV_255: &'static str = "The value is lower than 1/255!";
pub const VALUE_LOWER_MINUS_255: &'static str = "The value is lower than -255!";
pub const VALUE_NEGATIVE: &'static str = "The value is negative!";

// wrong format
pub const WRONG_FORMAT_PARAMETER: &'static str = "Wrong format of parameter: ";

pub fn print_error_and_quit(error: &'static str, error_value: Option<&str>) -> ! {
    print_error(error, error_value.unwrap_or(""));

    println!("{}", infos::QUITTED_PROGRAM);
    process::exit(1)
}

fn print_error(error: &'static str, error_value: &str) {
    eprint!("{}", error);
    eprintln!("{}", error_value);
}

pub fn print_warning(warning: &'static str) {
    eprint!("{}", warning);
}

