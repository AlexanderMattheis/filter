use clap::ArgMatches;

use crate::system::io::extracted_arguments::ExtractedArguments;
use crate::system::defaults::cli::commands;

pub fn extract(matches: &ArgMatches) -> ExtractedArguments {
    let input_path = match matches.value_of(commands::INPUT) {
        Some(input_path) => String::from(input_path),
        _ => String::new()
    };

    let filter = match matches.value_of(commands::FILTER) {
        Some(action) => String::from(action),
        _ => String::new()
    };

    let params = match matches.value_of(commands::PARAMS) {
        Some(action) => String::from(action),
        _ => String::new()
    };

    let output_path = match matches.value_of(commands::OUTPUT) {
        Some(output_path) => String::from(output_path),
        _ => String::new()
    };

    return ExtractedArguments { input_path: input_path, filter, params, output_path: output_path };
}
