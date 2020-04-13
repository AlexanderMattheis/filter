use crate::system::data::composed::point_operations::contrast_input::ContrastInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::filters_params_defaults::ContrastDefaults;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> ContrastInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => ContrastDefaults::CHANNELS_INPUT
    };

    let divide_by_value = match input.division {
        Some(value) => value,
        _ => false
    };

    let value = match input.value {
        Some(contrast) => if divide_by_value { 1.0 / contrast } else { contrast },
        _ => ContrastDefaults::VALUE
    };

    return ContrastInput { channels, value };
}