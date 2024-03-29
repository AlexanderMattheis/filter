use crate::system::data::composed::point_operations::brightness_input::BrightnessInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::BrightnessDefauls;
use crate::system::io::input::_basic_parser;
use crate::system::defaults::messages::errors;

pub fn parse_params(params: &String) -> BrightnessInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => BrightnessDefauls::CHANNELS_INPUT
    };

    let value = match input.value {
        Some(brightness) => brightness as i16,  // that is ok, since f64 should contain i16
        _ => BrightnessDefauls::VALUE
    };

    validate_input(value);
    return BrightnessInput { channels, value };
}

fn validate_input(value: i16) {
    if value < -255 {
        errors::print_error_and_quit(errors::VALUE_LOWER_MINUS_255, None);
    } else if value > 255 {
        errors::print_error_and_quit(errors::VALUE_HIGHER_255, None);
    }
}
