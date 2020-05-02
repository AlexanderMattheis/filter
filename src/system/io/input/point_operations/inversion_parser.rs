use crate::system::data::composed::point_operations::inversion_input::InversionInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::InversionDefaults;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> InversionInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => InversionDefaults::CHANNELS_INPUT
    };

    return InversionInput { channels };
}