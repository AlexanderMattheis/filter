use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::logic::actions::filters::border_handling;
use crate::logic::data_structures::patch::Patch1D;
use crate::system::data::composed::filters::linear::_linear_input::LinearInput;
use crate::system::data::composed::filters::linear::box_blur_input::BoxBlurInput;
use std::time::Instant;

/*
Hint: Could be optimized by about 20% if not the Patch1D data structure but only indices are used to read out a certain pixel value.
*/
pub fn run(image: &DynamicImage, temp_image_1: &mut DynamicImage, temp_image_2: &mut DynamicImage, input_params: &BoxBlurInput) {
    for i in 0..input_params.iterations {
        if i == 0 {
            blur_horizontally(image, temp_image_1, &input_params.linear_input);
            blur_vertically(temp_image_1, temp_image_2, &input_params.linear_input);
        } else {
            blur_horizontally(temp_image_2, temp_image_1, &input_params.linear_input);
            blur_vertically(temp_image_1, temp_image_2, &input_params.linear_input);
        }
    }
}

fn blur_horizontally(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &LinearInput) {
    let dimensions = image.dimensions();
    let mut patch_horizontal = Patch1D::new(2 * input_params.radius_horizontal + 1, None);

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {  // '-patch_width' due to way border-handling is handled
            let mut pixel_value = empty_image.get_pixel(u, v).0;
            let original_pixel_value = image.get_pixel(u, v).0;

            fill_patch_horizontally(&image, &mut patch_horizontal, input_params, u, v, dimensions.0);
            set_pixel_values(&mut pixel_value, &original_pixel_value, &patch_horizontal, input_params);
            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: pixel_value });
        }
        patch_horizontal.clear();  // in a new line there are new values
    }
}

fn fill_patch_horizontally(image: &DynamicImage, patch: &mut Patch1D, input_params: &LinearInput, pos_u: u32, pos_v: u32, image_width: u32) {
    let radius = input_params.radius_horizontal as u32;

    if pos_u == 0 {
        border_handling::init_patch(image, patch, input_params, pos_u, pos_v, radius, image_width, true);
    } else if pos_u + radius >= image_width {
        border_handling::handle_border_at_end(image, patch, input_params, pos_u, pos_v, radius, image_width, true);  // for more efficiency
    } else {
        set_patch_horizontally(image, patch, input_params, pos_u, pos_v, radius);
    }
}

fn set_patch_horizontally(image: &DynamicImage, patch: &mut Patch1D, input_params: &LinearInput, pos_u: u32, pos_v: u32, radius: u32) {
    let pixel_value = image.get_pixel((pos_u + radius) as u32, pos_v as u32).0;
    insert_rgba_values(patch, input_params, &pixel_value);
}

fn insert_rgba_values(patch: &mut Patch1D, input_params: &LinearInput, pixel_value: &[u8; 4]) {
    if input_params.channels.red {
        patch.insert_red_at_back(pixel_value[0]);
    }

    if input_params.channels.green {
        patch.insert_green_at_back(pixel_value[1]);
    }

    if input_params.channels.blue {
        patch.insert_blue_at_back(pixel_value[2]);
    }

    if input_params.channels.alpha {
        patch.insert_alpha_at_back(pixel_value[3]);
    }
}

fn set_pixel_values(pixel_value: &mut [u8; 4], original_pixel_value: &[u8; 4], patch: &Patch1D, input_params: &LinearInput) {
    set_pixel_value(pixel_value, original_pixel_value, patch.average_red(), input_params.channels.red, 0);
    set_pixel_value(pixel_value, original_pixel_value, patch.average_green(), input_params.channels.green, 1);
    set_pixel_value(pixel_value, original_pixel_value, patch.average_blue(), input_params.channels.blue, 2);
    set_pixel_value(pixel_value, original_pixel_value, patch.average_alpha(), input_params.channels.alpha, 3);
}

fn set_pixel_value(pixel_value: &mut [u8; 4], original_pixel_value: &[u8; 4], patch_value: u8, channel_active: bool, channel_index: usize) {
    if channel_active {
        pixel_value[channel_index] = patch_value;
    } else {
        pixel_value[channel_index] = original_pixel_value[channel_index];
    }
}

fn blur_vertically(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &LinearInput) {
    let dimensions = image.dimensions();
    let mut patch_vertical = Patch1D::new(2 * input_params.radius_vertical + 1, None);

    for u in 0..dimensions.0 {
        for v in 0..dimensions.1 {
            let mut pixel_value = empty_image.get_pixel(u as u32, v as u32).0;
            let original_pixel_value = image.get_pixel(u as u32, v as u32).0;

            fill_patch_vertically(&image, &mut patch_vertical, input_params, u, v, dimensions.1);
            set_pixel_values(&mut pixel_value, &original_pixel_value, &patch_vertical, input_params);
            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: pixel_value });
        }
        patch_vertical.clear();  // in a new line there are new values
    }
}

fn fill_patch_vertically(image: &DynamicImage, patch: &mut Patch1D, input_params: &LinearInput, pos_u: u32, pos_v: u32, image_height: u32) {
    let radius = input_params.radius_vertical as u32;

    if pos_v == 0 {
        border_handling::init_patch(image, patch, input_params, pos_u, pos_v, radius, image_height, false);
    } else if pos_v + radius >= image_height {
        border_handling::handle_border_at_end(image, patch, input_params, pos_u, pos_v, radius, image_height, false);  // for more efficiency
    } else {
        set_patch_vertically(image, patch, input_params, pos_u, pos_v, radius);
    }
}

fn set_patch_vertically(image: &DynamicImage, patch: &mut Patch1D, input_params: &LinearInput, pos_u: u32, pos_v: u32, radius: u32) {
    let pixel_value = image.get_pixel(pos_u as u32, (pos_v + radius) as u32).0;
    insert_rgba_values(patch, input_params, &pixel_value);
}
