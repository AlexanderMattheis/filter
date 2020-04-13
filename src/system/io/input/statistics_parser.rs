use crate::system::defaults::cli::filters_params_defaults::StatisticsDefaults;
use crate::system::io::input::_basic_parser;
use crate::system::data::elementary::input::Input;
use crate::system::data::composed::statistics_input::StatisticsInput;

pub fn parse_params(params: &String) -> StatisticsInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => channels,
        _ => StatisticsDefaults::CHANNELS_INPUT
    };

    return StatisticsInput { channels };
}