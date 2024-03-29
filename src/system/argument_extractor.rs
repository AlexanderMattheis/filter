use clap::ArgMatches;

use crate::system::defaults::cli::commands;
use crate::system::extracted_arguments::ExtractedArguments;

pub fn extract(matches: &ArgMatches) -> ExtractedArguments {
    let input_path = match matches.value_of(commands::INPUT) {
        Some(input_path) => String::from(input_path),
        _ => String::new()
    };

    let reference_path = match matches.value_of(commands::REFERENCE) {
        Some(reference_path) => String::from(reference_path),
        _ => String::new()
    };

    let filter = match matches.value_of(commands::ACTION) {
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

    let manual = match matches.value_of(commands::MANUAL) {
        Some(manual) => String::from(manual),
        _ => String::new()
    };

    return ExtractedArguments { input_path, reference_path, filter, params, output_path, manual };
}
