use crate::system::data::composed::point_operations::brightness_input::BrightnessInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::filters_params_defaults::BrightnessDefauls;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> BrightnessInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => BrightnessDefauls::CHANNELS_INPUT
    };

    let value = match input.value {
        Some(brightness) => brightness,
        _ => BrightnessDefauls::VALUE
    };

    return BrightnessInput { channels, value };
}
