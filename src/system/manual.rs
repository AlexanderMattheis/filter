use std::process;

use crate::system::data::elementary::channels_input::{ChannelsInput, RgbaChannelsInput};
use crate::system::defaults::cli::{filters, filters_params};
use crate::system::defaults::cli::filters_params_defaults::{AutoContrastDefaults, BrightnessDefauls, ContrastDefaults, GammaDefaults, HistogramDefaults, HistogramEqualizationDefaults, HistogramSpecificationDefaults, InversionDefaults, LinearBlendingDefaults, StatisticsDefaults, ThresholdDefaults};
use crate::system::defaults::cli::filters_params_values::ChannelTypes;
use crate::system::defaults::cli::manuals;
use crate::system::defaults::messages::errors;

pub fn check(manual_name: &String) {
    match manual_name.as_str() {
        manuals::ALL => {
            print_basic();
            println!();
            print_point_operations();
        }
        manuals::BASIC => print_basic(),
        manuals::POINT_OPERATIONS => print_point_operations(),
        "" => {}
        _ => errors::print_error_and_quit(errors::NOT_EXISTENT_MANUAL, None)
    }

    if !manual_name.eq("") {
        process::exit(1);
    }
}

fn print_basic() {
    println!("BASIC: (defaults)");

    println!("image-manipulation --action \"{action}\" --params \"{channels}; {cumulative}; {logarithmic}\" --input <input> --output <output>",
             action = filters::HISTOGRAM,
             channels = [filters_params::CHANNELS, "=", get_channels(&HistogramDefaults::CHANNELS_INPUT).as_str()].concat(),
             cumulative = [filters_params::CUMULATIVE, "=", get_bool(HistogramDefaults::CUMULATIVE).as_str()].concat(),
             logarithmic = [filters_params::LOGARITHMIC, "=", get_bool(HistogramDefaults::LOGARITHMIC).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}\" --input <input> --output <output>",
             action = filters::STATISTICS,
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

    println!("image-manipulation --action \"{action}\" --params \"{channels}; {per_channel}; {quantile_low}; {quantile_high}\" --input <input> --output <output>",
             action = filters::AUTO_CONTRAST,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&AutoContrastDefaults::CHANNELS_INPUT).as_str()].concat(),
             per_channel = [filters_params::PER_CHANNEL, "=", get_bool(AutoContrastDefaults::PER_CHANNEL).as_str()].concat(),
             quantile_low = [filters_params::QUANTILE_LOW, "=", get_double(AutoContrastDefaults::QUANTILE_LOW).as_str()].concat(),
             quantile_high = [filters_params::QUANTILE_HIGH, "=", get_double(AutoContrastDefaults::QUANTILE_HIGH).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}; {value}\" --input <input> --output <output>",
             action = filters::BRIGHTNESS,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&BrightnessDefauls::CHANNELS_INPUT).as_str()].concat(),
             value = [filters_params::VALUE, "=", get_int_16(BrightnessDefauls::VALUE).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}; {value}\" --input <input> --output <output>",
             action = filters::CONTRAST,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&ContrastDefaults::CHANNELS_INPUT).as_str()].concat(),
             value = [filters_params::VALUE, "=", get_double(ContrastDefaults::VALUE).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}; {value}\" --input <input> --output <output>",
             action = filters::GAMMA,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&GammaDefaults::CHANNELS_INPUT).as_str()].concat(),
             value = [filters_params::VALUE, "=", get_double(GammaDefaults::VALUE).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}; {enhanced}; {per_channel}\" --input <input> --output <output>",
             action = filters::HISTOGRAM_EQUALIZATION,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&HistogramEqualizationDefaults::CHANNELS_INPUT).as_str()].concat(),
             enhanced = [filters_params::ENHANCED, "=", get_bool(HistogramEqualizationDefaults::ENHANCED).as_str()].concat(),
             per_channel = [filters_params::PER_CHANNEL, "=", get_bool(HistogramEqualizationDefaults::PER_CHANNEL).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}\" --input <input> --reference <reference> --output <output>",
             action = filters::HISTOGRAM_SPECIFICATION,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&HistogramSpecificationDefaults::CHANNELS_INPUT).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}\" --input <input> --output <output>",
             action = filters::INVERSION,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&InversionDefaults::CHANNELS_INPUT).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{channels}; {value}\" --input <input> --reference <reference> --output <output>",
             action = filters::LINEAR_BLENDING,
             channels = [filters_params::CHANNELS, "=", get_rgba_channels(&LinearBlendingDefaults::CHANNELS_INPUT).as_str()].concat(),
             value = [filters_params::VALUE, "=", get_double(LinearBlendingDefaults::VALUE).as_str()].concat()
    );

    println!("image-manipulation --action \"{action}\" --params \"{minimum}; {maximum}; {threshold}\" --input <input> --output <output>",
             action = filters::THRESHOLD,
             minimum = [filters_params::MINIMUM, "=", get_uint_8(ThresholdDefaults::MINIMUM).as_str()].concat(),
             maximum = [filters_params::MAXIMUM, "=", get_uint_8(ThresholdDefaults::MAXIMUM).as_str()].concat(),
             threshold = [filters_params::THRESHOLD, "=", get_uint_8(ThresholdDefaults::THRESHOLD).as_str()].concat()
    );

    println!("\nwhere:\n\
    {brightness} = <-255|..|255>; {contrast} = <0.0|/255.0|..|*255.0>; {channels} = <r|g|..|rgb|..|rgba>; \n\
    {enhanced} = <false|true>; {gamma} = <0.0..>; {linear_blending} = <0.0|..|1.0>; {minimum} = <0|..|255>; {maximum} = <0|..|255>; \n\
    {per_channel} = <false|true>; {quantile_low} = <0.0|..|1.0>; {quantile_high} = <0.0|..|1.0>;  \n\
    {threshold} = <0|..|255>;",
             brightness = "{brightness}",
             contrast = "{contrast}",
             channels = filters_params::CHANNELS,
             enhanced = filters_params::ENHANCED,
             gamma = "{gamma}",
             linear_blending = "{linear-blending}",
             minimum = filters_params::MINIMUM,
             maximum = filters_params::MAXIMUM,
             per_channel = filters_params::PER_CHANNEL,
             quantile_low = filters_params::QUANTILE_LOW,
             quantile_high = filters_params::QUANTILE_HIGH,
             threshold = filters_params::THRESHOLD
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

fn get_bool(value: bool) -> String {
    return value.to_string();
}

fn get_double(value: f64) -> String {
    return value.to_string();
}

fn get_int_16(value: i16) -> String {
    return value.to_string();
}

fn get_uint_8(value: u8) -> String {
    return value.to_string();
}
