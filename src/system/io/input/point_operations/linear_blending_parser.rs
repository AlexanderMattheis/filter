use crate::system::data::composed::point_operations::linear_blending_input::LinearBlendingInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::filters_params_defaults::LinearBlendingDefaults;
use crate::system::io::input::_basic_parser;
use crate::system::defaults::messages::errors;

pub fn parse_params(params: &String) -> LinearBlendingInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => LinearBlendingDefaults::CHANNELS_INPUT
    };

    let value = match input.value {
        Some(alpha) => alpha,
        _ => LinearBlendingDefaults::VALUE
    };

    validate_input(value);
    return LinearBlendingInput { channels, value };
}

fn validate_input(value: f64) {
    if value < 0.0 {
        errors::print_error_and_quit(errors::VALUE_NEGATIVE, None);
    } else if value > 1.0 {
        errors::print_error_and_quit(errors::VALUE_HIGHER_1, None);
    }
}