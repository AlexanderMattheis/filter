use crate::system::data::composed::point_operations::histogram_specification_input::HistogramSpecificationInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::filters_params_defaults::{HistogramEqualizationDefaults, HistogramSpecificationDefaults};
use crate::system::defaults::messages::errors;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> HistogramSpecificationInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => HistogramSpecificationDefaults::CHANNELS_INPUT
    };

    let per_channel = match input.per_channel {
        Some(per_channel) => per_channel,
        _ => HistogramSpecificationDefaults::PER_CHANNEL
    };

    return HistogramSpecificationInput { channels, per_channel };
}