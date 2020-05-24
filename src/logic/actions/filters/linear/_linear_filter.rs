use image::{DynamicImage, GenericImageView};

use crate::logic::actions::filters::border_handling::BorderHandling;
use crate::logic::data_structures::patches::average_patch::AveragePatch1D;
use crate::system::data::composed::filters::filter_input::FilterInput;

pub fn fill_patch_horizontally(image: &DynamicImage, border_handling: &BorderHandling, patch: &mut AveragePatch1D, filter_input: &FilterInput, pos_u: u32, pos_v: u32, image_width: u32) {
    let radius = filter_input.radius_horizontal as u32;

    if pos_u == 0 {
        init_patch_horizontally(image, border_handling, &(pos_u as i32, pos_v as i32), image_width as i32, patch, filter_input);
    } else if pos_u + radius >= image_width {
        insert_rgba_values(patch, filter_input, &(border_handling.get_pixel)(image, (pos_u + radius) as i32, pos_v as i32, &filter_input.background_color));
    } else {
        set_patch_horizontally(image, patch, filter_input, pos_u, pos_v, radius);
    }
}

fn init_patch_horizontally(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                           dimension: i32, patch: &mut AveragePatch1D, filter_params: &FilterInput) {
    let horizontal = filter_params.radius_horizontal as i32;
    let mut pixel_value;

    for j in -horizontal..=horizontal {
        if position.0 + j < 0 || position.0 + j >= dimension {
            pixel_value = (border_handling.get_pixel)(image, position.0 + j, position.1, &filter_params.background_color);
        } else {
            pixel_value = image.get_pixel((position.0 + j) as u32, position.1 as u32).0;
        }

        insert_rgba_values(patch, filter_params, &pixel_value);
    }
}

fn insert_rgba_values(patch: &mut AveragePatch1D, filter_input: &FilterInput, pixel_value: &[u8; 4]) {
    if filter_input.channels.red {
        patch.insert_red_at_back(pixel_value[0]);
    }

    if filter_input.channels.green {
        patch.insert_green_at_back(pixel_value[1]);
    }

    if filter_input.channels.blue {
        patch.insert_blue_at_back(pixel_value[2]);
    }

    if filter_input.channels.alpha {
        patch.insert_alpha_at_back(pixel_value[3]);
    }
}

fn set_patch_horizontally(image: &DynamicImage, patch: &mut AveragePatch1D, input_params: &FilterInput, pos_u: u32, pos_v: u32, radius: u32) {
    let pixel_value = image.get_pixel((pos_u + radius) as u32, pos_v as u32).0;
    insert_rgba_values(patch, input_params, &pixel_value);
}

pub fn fill_patch_vertically(image: &DynamicImage, border_handling: &BorderHandling, patch: &mut AveragePatch1D, filter_input: &FilterInput, pos_u: u32, pos_v: u32, image_height: u32) {
    let radius = filter_input.radius_vertical as u32;

    if pos_v == 0 {
        init_patch_vertically(image, border_handling, &(pos_u as i32, pos_v as i32), image_height as i32, patch, filter_input);
    } else if pos_v + radius >= image_height {
        insert_rgba_values(patch, filter_input, &(border_handling.get_pixel)(image, pos_u as i32, (pos_v + radius) as i32, &filter_input.background_color));
    } else {
        set_patch_vertically(image, patch, filter_input, pos_u, pos_v, radius);
    }
}

fn init_patch_vertically(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                         dimension: i32, patch: &mut AveragePatch1D, filter_params: &FilterInput) {
    let vertical = filter_params.radius_horizontal as i32;
    let mut pixel_value;

    for i in -vertical..=vertical {
        if position.1 + i < 0 || position.1 + i >= dimension {
            pixel_value = (border_handling.get_pixel)(image, position.0, position.1 + i, &filter_params.background_color);
        } else {
            pixel_value = image.get_pixel(position.0 as u32, (position.1 + i) as u32).0;
        }

        insert_rgba_values(patch, filter_params, &pixel_value);
    }
}

fn set_patch_vertically(image: &DynamicImage, patch: &mut AveragePatch1D, input_params: &FilterInput, pos_u: u32, pos_v: u32, radius: u32) {
    let pixel_value = image.get_pixel(pos_u as u32, (pos_v + radius) as u32).0;
    insert_rgba_values(patch, input_params, &pixel_value);
}