use image::{DynamicImage, GenericImageView};
use crate::logic::data_structures::patch::Patch1D;
use crate::system::data::composed::filters::linear::box_blur_input::BoxBlurInput;
use crate::system::defaults::types::border_handling_type::BorderHandlingType;

pub(crate) fn init_patch(image: &DynamicImage, patch: &mut Patch1D, input_params: &BoxBlurInput, pos_u: u32, pos_v: u32, radius: u32, image_dimension: u32, horizontally: bool) {
    match input_params.border_handling {
        BorderHandlingType::PaddingConstantValue => init_patch_constant_value_at_start(image, patch, input_params, pos_u, pos_v, radius, horizontally),
        BorderHandlingType::PaddingExtend => init_patch_extend_at_start(image, patch, pos_u, pos_v, radius, horizontally),
        BorderHandlingType::PaddingMirror => init_patch_mirror_at_start(image, patch, pos_u, pos_v, radius, horizontally),
        BorderHandlingType::PaddingPeriodically => init_patch_periodically_at_start(image, patch, pos_u, pos_v, radius, image_dimension, horizontally),
        _ => {}
    }
}

fn init_patch_constant_value_at_start(image: &DynamicImage, patch: &mut Patch1D, input_params: &BoxBlurInput, pos_u: u32, pos_v: u32, radius: u32, horizontally: bool) {
    // pixels outside the image
    for _i in 0..radius {
        patch.insert_red_at_back(input_params.background_color[0]);
        patch.insert_green_at_back(input_params.background_color[1]);
        patch.insert_blue_at_back(input_params.background_color[2]);
        patch.insert_alpha_at_back(input_params.background_color[3]);
    }

    init_inside_image_pixels(image, patch, pos_u, pos_v, radius, horizontally);
}

fn init_inside_image_pixels(image: &DynamicImage, patch: &mut Patch1D, pos_u: u32, pos_v: u32, radius: u32, horizontally: bool) {
    for i in 0..(radius + 1) {  // '+1' due to the patch center
        let pixel_value;
        if horizontally {
            pixel_value = image.get_pixel(i, pos_v).0;
        } else {
            pixel_value = image.get_pixel(pos_u, i).0;
        }

        patch.insert_red_at_back(pixel_value[0]);
        patch.insert_green_at_back(pixel_value[1]);
        patch.insert_blue_at_back(pixel_value[2]);
        patch.insert_alpha_at_back(pixel_value[3]);
    }
}

fn init_patch_extend_at_start(image: &DynamicImage, patch: &mut Patch1D, pos_u: u32, pos_v: u32, radius: u32, horizontally: bool) {
    let pixel_value;
    if horizontally {
        pixel_value = image.get_pixel(0, pos_v).0;
    } else {
        pixel_value = image.get_pixel(pos_u, 0).0;
    }

    // pixels outside the image
    for _i in 0..radius {
        patch.insert_red_at_back(pixel_value[0]);
        patch.insert_green_at_back(pixel_value[1]);
        patch.insert_blue_at_back(pixel_value[2]);
        patch.insert_alpha_at_back(pixel_value[3]);
    }

    init_inside_image_pixels(image, patch, pos_u, pos_v, radius, horizontally);
}

fn init_patch_mirror_at_start(image: &DynamicImage, patch: &mut Patch1D, pos_u: u32, pos_v: u32, radius: u32, horizontally: bool) {
    // pixels outside the image
    for r in 0..radius {
        let pixel_value;
        if horizontally {
            pixel_value = image.get_pixel(r + 1, pos_v).0;
        } else {
            pixel_value = image.get_pixel(pos_u, r + 1).0;
        }

        patch.insert_red_at_back(pixel_value[0]);
        patch.insert_green_at_back(pixel_value[1]);
        patch.insert_blue_at_back(pixel_value[2]);
        patch.insert_alpha_at_back(pixel_value[3]);
    }

    init_inside_image_pixels(image, patch, pos_u, pos_v, radius, horizontally);
}

fn init_patch_periodically_at_start(image: &DynamicImage, patch: &mut Patch1D, pos_u: u32, pos_v: u32, radius: u32, image_dimension: u32, horizontally: bool) {
    // pixels outside the image
    for r in radius..0 {
        let pixel_value;
        if horizontally {
            pixel_value = image.get_pixel(image_dimension - r, pos_v).0;
        } else {
            pixel_value = image.get_pixel(pos_u, image_dimension - r).0;
        }

        patch.insert_red_at_back(pixel_value[0]);
        patch.insert_green_at_back(pixel_value[1]);
        patch.insert_blue_at_back(pixel_value[2]);
        patch.insert_alpha_at_back(pixel_value[3]);
    }

    init_inside_image_pixels(image, patch, pos_u, pos_v, radius, horizontally);
}

pub(crate) fn handle_border_at_end(image: &DynamicImage, patch: &mut Patch1D, input_params: &BoxBlurInput, pos_u: u32, pos_v: u32, radius: u32, image_dimension: u32, horizontally: bool) {
    match input_params.border_handling {
        BorderHandlingType::PaddingConstantValue => create_padding_constant_value_at_end(patch, input_params),
        BorderHandlingType::PaddingExtend => create_padding_extend_at_end(image, patch, pos_u, pos_v, image_dimension, horizontally),
        BorderHandlingType::PaddingMirror => create_padding_mirror_at_end(image, patch, pos_u, pos_v, radius, image_dimension, horizontally),
        BorderHandlingType::PaddingPeriodically => create_padding_periodically_at_end(image, patch, pos_u, pos_v, radius, image_dimension, horizontally),
        _ => {}
    }
}

fn create_padding_constant_value_at_end(patch: &mut Patch1D, input_params: &BoxBlurInput) {
    patch.insert_red_at_back(input_params.background_color[0]);
    patch.insert_green_at_back(input_params.background_color[1]);
    patch.insert_blue_at_back(input_params.background_color[2]);
    patch.insert_alpha_at_back(input_params.background_color[3]);
}

fn create_padding_extend_at_end(image: &DynamicImage, patch: &mut Patch1D, pos_u: u32, pos_v: u32, image_dimension: u32, horizontally: bool) {
    let pixel_value;
    if horizontally {
        pixel_value = image.get_pixel(image_dimension - 1, pos_v).0;
    } else {
        pixel_value = image.get_pixel(pos_u, image_dimension - 1).0;
    }

    patch.insert_red_at_back(pixel_value[0]);
    patch.insert_green_at_back(pixel_value[1]);
    patch.insert_blue_at_back(pixel_value[2]);
    patch.insert_alpha_at_back(pixel_value[3]);
}

fn create_padding_mirror_at_end(image: &DynamicImage, patch: &mut Patch1D, pos_u: u32, pos_v: u32, radius: u32, image_dimension: u32, horizontally: bool) {
    let pixel_value;
    if horizontally {
        // '+1' since you start counting from zero
        let boxes_outside_image = pos_u + 1 + radius - image_dimension;
        let pos_mirror = (image_dimension - 1) - boxes_outside_image;
        pixel_value = image.get_pixel(pos_mirror, pos_v).0;
    } else {
        let boxes_outside_image = pos_v + 1 + radius - image_dimension;
        let pos_mirror = (image_dimension - 1) - boxes_outside_image;
        pixel_value = image.get_pixel(pos_u, pos_mirror).0;
    }

    patch.insert_red_at_back(pixel_value[0]);
    patch.insert_green_at_back(pixel_value[1]);
    patch.insert_blue_at_back(pixel_value[2]);
    patch.insert_alpha_at_back(pixel_value[3]);
}

fn create_padding_periodically_at_end(image: &DynamicImage, patch: &mut Patch1D, pos_u: u32, pos_v: u32, radius: u32, image_dimension: u32, horizontally: bool) {
    // can be solved with *MATHEMATICAL* modulo, but this should be less performant
    let pixel_value;

    if horizontally {
        let pos_from_start = pos_u + radius - image_dimension;
        pixel_value = image.get_pixel(pos_from_start, pos_v).0;
    } else {
        let pos_from_start = pos_v + radius - image_dimension;
        pixel_value = image.get_pixel(pos_u, pos_from_start).0;
    }

    patch.insert_red_at_back(pixel_value[0]);
    patch.insert_green_at_back(pixel_value[1]);
    patch.insert_blue_at_back(pixel_value[2]);
    patch.insert_alpha_at_back(pixel_value[3]);
}