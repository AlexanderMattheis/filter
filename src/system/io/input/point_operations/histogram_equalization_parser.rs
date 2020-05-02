use crate::system::data::composed::point_operations::histogram_equalization_input::HistogramEqualizationInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::HistogramEqualizationDefaults;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> HistogramEqualizationInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => HistogramEqualizationDefaults::CHANNELS_INPUT
    };

    let enhanced = match input.enhanced {
        Some(enhanced) => enhanced,
        _ => HistogramEqualizationDefaults::ENHANCED
    };

    let per_channel = match input.per_channel {
        Some(per_channel) => per_channel,
        _ => HistogramEqualizationDefaults::PER_CHANNEL
    };

    return HistogramEqualizationInput { channels, accumulate_roots: enhanced, per_channel };
}