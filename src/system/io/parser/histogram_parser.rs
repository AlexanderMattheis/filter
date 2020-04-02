use crate::system::defaults::cli::filters_params_defaults::HistogramDefaults;
use crate::system::io::data::composed::histogram_input::HistogramInput;
use crate::system::io::data::elementary::input::Input;
use crate::system::io::parser::_basic_parser;

pub fn parse_params(params: &String) -> HistogramInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => channels,
        _ => HistogramDefaults::CHANNELS_INPUT
    };

    let logarithmic = match input.logarithmic {
        Some(logarithmic) => logarithmic,
        _ => HistogramDefaults::LOGARITHMIC
    };

    return HistogramInput { channels, logarithmic };
}

