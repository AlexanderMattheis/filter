use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::FilterDefaults;
use crate::system::defaults::messages::errors;

pub fn parse_params(input: &Input, image_dimensions: &(u32, u32)) -> FilterInput {
    let background_color = match input.background_color {
        Some(background_color) => background_color,
        _ => FilterDefaults::BACKGROUND_COLOR
    };

    let border_handling = match input.border_handling {
        Some(border_handling) => border_handling,
        _ => FilterDefaults::BORDER_HANDLING
    };

    let channels = match &input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => FilterDefaults::CHANNELS_INPUT
    };

    let radius_horizontal = match input.radius_horizontal {
        Some(radius_horizontal) => radius_horizontal,
        _ => FilterDefaults::RADIUS_HORIZONTAL
    };

    let radius_vertical = match input.radius_vertical {
        Some(radius_vertical) => radius_vertical,
        _ => FilterDefaults::RADIUS_VERTICAL
    };

    validate_input(radius_horizontal as u32, radius_vertical as u32, image_dimensions);
    return FilterInput {
        background_color,
        border_handling,
        channels,
        radius_horizontal,
        radius_vertical,
    };
}

fn validate_input(radius_horizontal: u32, radius_vertical: u32, image_dimensions: &(u32, u32)) {
    if radius_horizontal > image_dimensions.0 {
        errors::print_error_and_quit(errors::RADIUS_BIGGER_IMAGE_HORIZONTAL, None);
    }

    if radius_vertical > image_dimensions.1 {
        errors::print_error_and_quit(errors::RADIUS_BIGGER_IMAGE_VERTICAL, None);
    }
}
