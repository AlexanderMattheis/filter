use crate::system::data::composed::filters::linear::box_blur_input::BoxBlurInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::BoxBlurDefaults;
use crate::system::defaults::messages::errors;
use crate::system::io::input::_basic_parser;

pub fn parse_params(params: &String, image_dimensions: &(u32, u32)) -> BoxBlurInput {
    let input: Input = _basic_parser::parse_params(params);

    let background_color = match input.background_color {
        Some(background_color) => background_color,
        _ => BoxBlurDefaults::BACKGROUND_COLOR
    };

    let border_handling = match input.border_handling {
        Some(border_handling) => border_handling,
        _ => BoxBlurDefaults::BORDER_HANDLING
    };

    let channels = match input.channels {
        Some(channels) => RgbaChannelsInput::new(&channels),
        _ => BoxBlurDefaults::CHANNELS_INPUT
    };

    let iterations = match input.iterations {
        Some(iterations) => iterations,
        _ => BoxBlurDefaults::ITERATIONS
    };

    let radius_horizontal = match input.radius_horizontal {
        Some(radius_horizontal) => radius_horizontal,
        _ => BoxBlurDefaults::RADIUS_HORIZONTAL
    };

    let radius_vertical = match input.radius_vertical {
        Some(radius_vertical) => radius_vertical,
        _ => BoxBlurDefaults::RADIUS_VERTICAL
    };

    validate_input(radius_horizontal as u32, radius_vertical as u32, image_dimensions);
    return BoxBlurInput {
        background_color,
        border_handling,
        channels,
        iterations,
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

