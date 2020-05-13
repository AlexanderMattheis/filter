use image::{DynamicImage, GenericImageView};

use crate::logic::actions::filters::border_handling;
use crate::logic::data_structures::patch::Patch1D;
use crate::system::data::composed::filters::linear::_linear_input::LinearInput;

pub fn fill_patch_horizontally(image: &DynamicImage, patch: &mut Patch1D, input_params: &LinearInput, pos_u: u32, pos_v: u32, image_width: u32) {
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

pub fn fill_patch_vertically(image: &DynamicImage, patch: &mut Patch1D, input_params: &LinearInput, pos_u: u32, pos_v: u32, image_height: u32) {
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