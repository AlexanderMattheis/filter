use std::process;

use crate::system::data::elementary::channels_input::{ChannelsInput, RgbaChannelsInput};
use crate::system::defaults::cli::{filters, filters_params};
use crate::system::defaults::cli::filters_params_defaults::{AutoContrastDefaults, HistogramDefaults, StatisticsDefaults};
use crate::system::defaults::cli::filters_params_values::ChannelTypes;
use crate::system::defaults::cli::manuals;
use crate::system::defaults::messages::errors;

pub fn check(manual_name: &String) {
    match manual_name.as_str() {
        manuals::ALL => {
            print_basic();
            print_point_operations();
        }
        manuals::BASIC => print_basic(),
        manuals::POINT_OPERATIONS => print_point_operations(),
        "" => {}
        _ => errors::print_error_and_quit(errors::NOT_EXISTENT_MANUAL, None)
    }

    process::exit(1);
}

fn print_basic() {
    println!("BASIC: (defaults)");

    println!("image-manipulation --filter \"{filter}\" --params \"{channels}; {cumulative}; {logarithmic}\" --input <input> --output <output>",
             filter = filters::HISTOGRAM,
             channels = [filters_params::CHANNELS, "=", get_channels(&HistogramDefaults::CHANNELS_INPUT).as_str()].concat(),
             cumulative = [filters_params::CUMULATIVE, "=", get_boolean(HistogramDefaults::CUMULATIVE).as_str()].concat(),
             logarithmic = [filters_params::LOGARITHMIC, "=", get_boolean(HistogramDefaults::LOGARITHMIC).as_str()].concat()
    );

    println!("image-manipulation --filter \"{filter}\" --params \"{channels}\" --input <input> --output <output>",
             filter = filters::STATISTICS,
             channels = [filters_params::CHANNELS, "=", get_channels(&StatisticsDefaults::CHANNELS_INPUT).as_str()].concat()
    );

    println!("\nwhere: {channels} = <r|g|..|rgb|..|rgbal>; {cumulative} = <false|true>; {logarithmic} = <false|true>",
             channels = filters_params::CHANNELS,
             cumulative = filters_params::CUMULATIVE,
             logarithmic = filters_params::LOGARITHMIC
    );
}

fn print_point_operations() {
    println!("POINT_OPERATIONS: (defaults)");

    println!("image-manipulation --filter \"{filter}\" --params \"{channels}; {per_channel}; {quantile_low}; {quantile_high}\" --input <input> --output <output>",
             filter = filters::AUTO_CONTRAST,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&AutoContrastDefaults::CHANNELS_INPUT).as_str()].concat(),
             per_channel = [filters_params::PER_CHANNEL, "=", get_boolean(AutoContrastDefaults::PER_CHANNEL).as_str()].concat(),
             quantile_low = [filters_params::QUANTILE_LOW, "=", get_float(AutoContrastDefaults::QUANTILE_LOW).as_str()].concat(),
             quantile_high = [filters_params::QUANTILE_HIGH, "=", get_float(AutoContrastDefaults::QUANTILE_HIGH).as_str()].concat()
    );

    println!("\nwhere: {channels} = <r|g|..|rgb|..|rgba>;",
             channels = filters_params::CHANNELS
    );
}

fn get_channels(channels_input: &ChannelsInput) -> String {
    let mut channels = vec![];

    if channels_input.red {
        channels.push(ChannelTypes::RED.to_string());
    }

    if channels_input.green {
        channels.push(ChannelTypes::GREEN.to_string());
    }

    if channels_input.blue {
        channels.push(ChannelTypes::BLUE.to_string());
    }

    if channels_input.alpha {
        channels.push(ChannelTypes::ALPHA.to_string());
    }

    if channels_input.luminance {
        channels.push(ChannelTypes::LUMINANCE.to_string());
    }

    return channels.concat();
}

fn get_rgba_channels(channels_input: &RgbaChannelsInput) -> String {
    return get_channels(&ChannelsInput::new(channels_input));
}

fn get_boolean(value: bool) -> String {
    return value.to_string();
}

fn get_float(value: f64) -> String {
    return value.to_string();
}
