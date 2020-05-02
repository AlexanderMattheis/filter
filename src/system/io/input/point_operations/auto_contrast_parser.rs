use crate::system::data::composed::point_operations::auto_contrast_input::AutoContrastInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::AutoContrastDefaults;
use crate::system::defaults::messages::errors;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String) -> AutoContrastInput {
    let input: Input = _basic_parser::parse_params(params);

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => AutoContrastDefaults::CHANNELS_INPUT
    };

    let per_channel = match input.per_channel {
        Some(per_channel) => per_channel,
        _ => AutoContrastDefaults::PER_CHANNEL
    };

    let quantile_low = match input.quantile_low {
        Some(quantile_low) => quantile_low,
        _ => AutoContrastDefaults::QUANTILE_LOW
    };

    let quantile_high = match input.quantile_high {
        Some(quantile_high) => quantile_high,
        _ => AutoContrastDefaults::QUANTILE_HIGH
    };

    validate_input(quantile_low, quantile_high);
    return AutoContrastInput { channels, per_channel, quantile_low, quantile_high };
}

fn validate_input(quantile_low: f64, quantile_high: f64) {
    if quantile_low + quantile_high > 1.0 {
        errors::print_error_and_quit(errors::QUANTILES_SUM_HIGHER_1_0, None);
    }
}
