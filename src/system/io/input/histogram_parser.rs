use crate::system::defaults::cli::actions_params_defaults::HistogramDefaults;
use crate::system::io::input::_basic_parser;
use crate::system::data::composed::histogram_input::HistogramInput;
use crate::system::data::elementary::input::Input;

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

    let cumulative = match input.cumulative {
        Some(cumulative) => cumulative,
        _ => HistogramDefaults::CUMULATIVE
    };

    return HistogramInput { channels, logarithmic, cumulative };
}

