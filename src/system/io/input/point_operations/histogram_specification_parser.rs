use crate::system::data::composed::point_operations::histogram_specification_input::HistogramSpecificationInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::HistogramSpecificationDefaults;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> HistogramSpecificationInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => HistogramSpecificationDefaults::CHANNELS_INPUT
    };

    return HistogramSpecificationInput { channels };
}