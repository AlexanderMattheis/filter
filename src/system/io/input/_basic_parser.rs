use std::str::FromStr;

use crate::system::data::elementary::channels_input::ChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::filters_params;
use crate::system::defaults::cli::filters_params_values::ChannelTypes;
use crate::system::defaults::messages::errors;

const PARAMS_SEPARATOR: &str = ";";
const PARAM_NAME_VALUE_SEPARATOR: &str = "=";

pub fn parse_params(params: &String) -> Input {
    let splitted_params: Vec<&str> = params.split(PARAMS_SEPARATOR).collect();
    let mut input = Input {
        value: None,
        channels: None,
        logarithmic: None,
        cumulative: None,
    };

    for param in splitted_params {
        parse_param(param, &mut input);
    }

    return input;
}

fn parse_param(param: &str, input: &mut Input) {
    let splitted_param: Vec<&str> = param.split(PARAM_NAME_VALUE_SEPARATOR).collect();

    if splitted_param.len() != 2 {
        errors::print_error_and_quit(errors::PARAMETER_WRONG_FORMAT, Some(param));
    }

    let param_name = match splitted_param.get(0) {
        Some(name) => name.trim(),
        None => ""
    };

    let param_value = match splitted_param.get(1) {
        Some(value) => value.trim(),
        None => ""
    };

    match param_name {
        filters_params::CHANNELS => input.channels = Some(parse_channels_value(param_value)),
        filters_params::LOGARITHMIC => input.logarithmic = Some(parse_boolean_value(param_value)),
        filters_params::CUMULATIVE => input.cumulative = Some(parse_boolean_value(param_value)),
        filters_params::VALUE => input.value = Some(parse_integer_value(param_value)),
        _ => {}
    };
}

fn parse_channels_value(channels_value: &str) -> ChannelsInput {
    let mut red = false;
    let mut green = false;
    let mut blue = false;
    let mut alpha = false;
    let mut luminance = false;

    let mut tmp = [0; 4];

    for char in channels_value.chars() {
        match char {
            ChannelTypes::RED => red = true,
            ChannelTypes::GREEN => green = true,
            ChannelTypes::BLUE => blue = true,
            ChannelTypes::ALPHA => alpha = true,
            ChannelTypes::LUMINANCE => luminance = true,
            _ => {
                errors::print_error_and_quit(errors::NOT_EXISTENT_CHANNEL, Some(char.encode_utf8(&mut tmp)));
            }
        }
    }

    return ChannelsInput { red, green, blue, alpha, luminance };
}

fn parse_boolean_value(logarithmic_value: &str) -> bool {
    if logarithmic_value == "true" {
        return true;
    } else if logarithmic_value == "false" {
        return false;
    }

    errors::print_error_and_quit(errors::NOT_EXISTENT_BOOLEAN, Some(logarithmic_value));
}

fn parse_integer_value(number_value: &str) -> i16 {
    let value = number_value.parse::<i16>();

    if value.is_ok() {
        return value.unwrap();
    }

    errors::print_error_and_quit(errors::NOT_VALID_INTEGER, Some(number_value));
}
