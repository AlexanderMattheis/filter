use crate::system::data::composed::point_operations::gamma_input::GammaInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::GammaDefaults;
use crate::system::defaults::messages::errors;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> GammaInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => GammaDefaults::CHANNELS_INPUT
    };

    let value = match input.value {
        Some(value) => value,
        _ => GammaDefaults::VALUE
    };

    validate_input(value);
    return GammaInput { channels, value };
}

fn validate_input(value: f64) {
    if value < 0.0 {
        errors::print_error_and_quit(errors::VALUE_NEGATIVE, None);
    }
}