use crate::system::data::elementary::channels_input::ChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params;
use crate::system::defaults::cli::actions_params_values::ChannelTypes;
use crate::system::defaults::messages::errors;

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
        actions_params::CHANNELS => input.channels = Some(parse_channels_value(param_value)),
        actions_params::LOGARITHMIC => input.logarithmic = Some(parse_boolean_value(param_value)),
        actions_params::CUMULATIVE => input.cumulative = Some(parse_boolean_value(param_value)),
        actions_params::ENHANCED => input.enhanced = Some(parse_boolean_value(param_value)),
        actions_params::MAXIMUM => input.maximum = Some(parse_uint_8(param_value)),
        actions_params::MINIMUM => input.minimum = Some(parse_uint_8(param_value)),
        actions_params::PER_CHANNEL => input.per_channel = Some(parse_boolean_value(param_value)),
        actions_params::QUANTILE_LOW => input.quantile_low = Some(parse_double_value(param_value)),
        actions_params::QUANTILE_HIGH => input.quantile_high = Some(parse_double_value(param_value)),
        actions_params::THRESHOLD => input.threshold = Some(parse_uint_8(param_value)),
        actions_params::VALUE => {
            input.division = Some(is_division_operation(param_value));
            input.value = Some(parse_double_value(get_number_string(param_value)))
        }
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

fn parse_uint_8(number_value: &str) -> u8 {
    let value = number_value.parse::<u8>();

    if value.is_ok() {
        return value.unwrap();
    }

    errors::print_error_and_quit(errors::NOT_VALID_NUMBER, Some(number_value));
}
