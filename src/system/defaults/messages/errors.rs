use crate::system::defaults::messages::infos;
use std::process;

pub const NOT_EXISTENT_BOOLEAN: &'static str = "Not existent boolean: ";
pub const NOT_EXISTENT_CHANNEL: &'static str = "Not existent channel: ";
pub const NOT_EXISTENT_FILTER: &'static str = "Not existent filter: ";

pub const PARAMETER_WRONG_FORMAT: &'static str = "Wrong format of parameter: ";

pub fn print_error_and_quit(error: &'static str, error_message: &str) -> ! {
    print_error(error, error_message);

    println!("{}", infos::QUITTED_PROGRAM);
    process::exit(1)
}

fn print_error(error: &'static str, error_message: &str) {
    eprint!("{}", error);
    eprintln!("{}", error_message);
}

