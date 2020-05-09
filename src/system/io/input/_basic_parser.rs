use std::collections::BTreeSet;
use std::iter::FromIterator;

use crate::system::data::elementary::channels_input::ChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::{actions_params, actions_params_values};
use crate::system::defaults::cli::actions_params_values::{BorderHandlingTypes, ChannelTypes};
use crate::system::defaults::messages::{errors, infos};
use crate::system::defaults::types::border_handling_type::BorderHandlingType;

pub fn parse_params(params: &String) -> Input {
    if params.is_empty() {
        return Input::new();
    }

    let splitted_params: Vec<&str> = params.split(";").collect();
    let mut input = Input::new();

    for param in splitted_params {
        parse_param(param, &mut input);
    }

    return input;
}

fn parse_param(param: &str, input: &mut Input) {
    let splitted_param: Vec<&str> = param.split("=").collect();

    if splitted_param.len() != 2 {
        errors::print_error_and_quit(errors::WRONG_FORMAT_PARAMETER, Some(param));
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
        actions_params::BACKGROUND_COLOR => input.background_color = Some(parse_background_color(param_value)),
        actions_params::BORDER_HANDLING => input.border_handling = Some(parse_border_handling(param_value)),
        actions_params::CHANNELS => input.channels = Some(parse_channels_value(param_value)),
        actions_params::LOGARITHMIC => input.logarithmic = Some(parse_boolean_value(param_value)),
        actions_params::CUMULATIVE => input.cumulative = Some(parse_boolean_value(param_value)),
        actions_params::ENHANCED => input.enhanced = Some(parse_boolean_value(param_value)),
        actions_params::ITERATIONS => input.iterations = Some(parse_uint_16_value(param_value)),
        actions_params::MAXIMUM => input.maximum = Some(parse_uint_8_value(param_value)),
        actions_params::MINIMUM => input.minimum = Some(parse_uint_8_value(param_value)),
        actions_params::PER_CHANNEL => input.per_channel = Some(parse_boolean_value(param_value)),
        actions_params::QUANTILE_LOW => input.quantile_low = Some(parse_double_value(param_value)),
        actions_params::QUANTILE_HIGH => input.quantile_high = Some(parse_double_value(param_value)),
        actions_params::RADIUS_HORIZONTAL => input.radius_horizontal = Some(parse_usize_value(param_value)),
        actions_params::RADIUS_VERTICAL => input.radius_vertical = Some(parse_usize_value(param_value)),
        actions_params::THRESHOLD => input.threshold = Some(parse_uint_8_value(param_value)),
        actions_params::VALUE => {
            input.division = Some(is_division_operation(param_value));
            input.value = Some(parse_double_value(get_number_string(param_value)))
        }
        _ => {}
    };
}

fn parse_background_color(hex_color: &str) -> [u8; 4] {
    let wrong_prefix = !hex_color.starts_with("#");
    let wrong_length = hex_color.len() != 7 && hex_color.len() != 9;
    let not_hexadecimal = !BTreeSet::<char>::from_iter(hex_color[1..].chars())
        .is_subset(&BTreeSet::<char>::from_iter(actions_params_values::HEXADECIMAL_CHARACTERS.chars()));

    if wrong_prefix || wrong_length || not_hexadecimal {
        println!("{}", infos::VALID_COLOR);
        errors::print_error_and_quit(errors::NOT_VALID_COLOR, Some(hex_color));
    }

    return to_decimals(hex_color);
}

#[inline]
fn to_decimals(hex_color: &str) -> [u8; 4] {
    let mut hex_color_index = 1;  // skip '#'
    let mut rgba_color_index = 0;
    let mut rgba_color: [u8; 4] = [255; 4];  // avoids setting alpha-value

    while hex_color_index < hex_color[1..].len() {
        // cut out strings of length two for the different color values (rgba)
        rgba_color[rgba_color_index] = to_decimal(&hex_color[hex_color_index..hex_color_index + 2], 16);
        hex_color_index = hex_color_index + 2;
        rgba_color_index = rgba_color_index + 1;
    }

    return rgba_color;
}

fn to_decimal(number: &str, number_base: u32) -> u8 {
    let mut sum = 0;
    let mut power = number.len() as u32;

    for symbol in number.chars() {
        power -= 1;
        sum = sum + symbol.to_digit(number_base).unwrap() * number_base.pow(power);
    }

    return sum as u8;
}

fn parse_border_handling(border_handling: &str) -> BorderHandlingType {
    return match border_handling {
        BorderHandlingTypes::CONSTANT_VALUE => BorderHandlingType::ConstantValue,
        BorderHandlingTypes::UNPROCESSED => BorderHandlingType::Unprocessed,
        BorderHandlingTypes::PADDING_CONSTANT_VALUE => BorderHandlingType::PaddingConstantValue,
        BorderHandlingTypes::PADDING_EXTEND => BorderHandlingType::PaddingExtend,
        BorderHandlingTypes::PADDING_MIRROR => BorderHandlingType::PaddingMirror,
        BorderHandlingTypes::PADDING_PERIODICALLY => BorderHandlingType::PaddingPeriodically,
        _ => {
            errors::print_error_and_quit(errors::NOT_VALID_BORDER_HANDLING, Some(border_handling));
        }
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
                errors::print_error_and_quit(errors::NOT_VALID_CHANNEL, Some(char.encode_utf8(&mut tmp)));
            }
        }
    }

    return ChannelsInput { red, green, blue, alpha, luminance };
}

fn parse_boolean_value(boolean_value: &str) -> bool {
    if boolean_value == "true" {
        return true;
    } else if boolean_value == "false" {
        return false;
    }

    errors::print_error_and_quit(errors::NOT_VALID_BOOLEAN, Some(boolean_value));
}

fn is_division_operation(operation_and_number_value: &str) -> bool {
    let operation = match operation_and_number_value.chars().nth(0) {
        Some(char) => char,
        None => ' '
    };

    let irrelevant_possibilities = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*'];

    if irrelevant_possibilities.contains(&operation) {
        return false;
    } else if operation == '/' {
        return true;
    }

    errors::print_error_and_quit(errors::NOT_VALID_OPERATION, Some(operation.to_string().as_ref()));
}

fn get_number_string(operation_and_number_value: &str) -> &str {
    let operation = match operation_and_number_value.chars().nth(0) {
        Some(char) => char,
        None => ' '
    };

    let irrelevant_possibilities = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-'];

    if irrelevant_possibilities.contains(&operation) {
        return operation_and_number_value;
    } else if operation == '/' || operation == '*' {
        return &operation_and_number_value[1..];
    }

    errors::print_error_and_quit(errors::NOT_VALID_NUMBER_STRING, Some(operation_and_number_value));
}

fn parse_double_value(number_value: &str) -> f64 {
    let value = number_value.parse::<f64>();

    if value.is_ok() {
        return value.unwrap();
    }

    errors::print_error_and_quit(errors::NOT_VALID_NUMBER, Some(number_value));
}

fn parse_uint_8_value(number_value: &str) -> u8 {
    let value = number_value.parse::<u8>();

    if value.is_ok() {
        return value.unwrap();
    }

    errors::print_error_and_quit(errors::NOT_VALID_NUMBER, Some(number_value));
}

fn parse_uint_16_value(number_value: &str) -> u16 {
    let value = number_value.parse::<u16>();

    if value.is_ok() {
        return value.unwrap();
    }

    errors::print_error_and_quit(errors::NOT_VALID_NUMBER, Some(number_value));
}

fn parse_usize_value(number_value: &str) -> usize {
    let value = number_value.parse::<usize>();

    if value.is_ok() {
        return value.unwrap();
    }

    errors::print_error_and_quit(errors::NOT_VALID_NUMBER, Some(number_value));
}
