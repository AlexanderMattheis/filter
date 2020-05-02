use crate::system::data::composed::point_operations::threshold_input::ThresholdInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::ThresholdDefaults;
use crate::system::defaults::messages::errors;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> ThresholdInput {
    let input: Input = _basic_parser::parse_params(params);

    let threshold = match input.threshold {
        Some(threshold) => threshold,
        _ => ThresholdDefaults::THRESHOLD
    };

    let minimum = match input.minimum {
        Some(minimum) => minimum,
        _ => ThresholdDefaults::MINIMUM
    };

    let maximum = match input.maximum {
        Some(maximum) => maximum,
        _ => ThresholdDefaults::MAXIMUM
    };

    validate_input(minimum, maximum);
    return ThresholdInput { threshold, minimum, maximum };
}

fn validate_input(minimum: u8, maximum: u8) {
    if minimum > maximum {
        errors::print_error_and_quit(errors::MINIMUM_BIGGER_MAXIMUM, None);
    }
}