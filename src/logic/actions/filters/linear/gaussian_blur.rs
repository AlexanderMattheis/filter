use std::f64::consts::E;

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::logic::actions::filters::linear::_linear_filter;
use crate::logic::data_structures::kernel::Kernel1D;
use crate::logic::data_structures::patch::Patch1D;
use crate::system::data::composed::filters::linear::_linear_input::LinearInput;
use crate::system::data::composed::filters::linear::gaussian_blur_input::GaussianBlurInput;

pub fn run(image: &DynamicImage, temp_image_1: &mut DynamicImage, temp_image_2: &mut DynamicImage, input_params: &GaussianBlurInput) {
    blur_horizontally(image, temp_image_1, &input_params.linear_input);
    blur_vertically(temp_image_1, temp_image_2, &input_params.linear_input);
}

fn blur_horizontally(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &LinearInput) {
    let dimensions = image.dimensions();
    let kernel = compute_gaussian_kernel(input_params.radius_horizontal as i32);
    let mut patch_horizontal = Patch1D::new(input_params.radius_horizontal, Some(kernel));

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {  // '-patch_width' due to way border-handling is handled
            let mut pixel_value = empty_image.get_pixel(u, v).0;
            let original_pixel_value = image.get_pixel(u, v).0;

            _linear_filter::fill_patch_horizontally(&image, &mut patch_horizontal, input_params, u, v, dimensions.0);
            set_pixel_values(&mut pixel_value, &original_pixel_value, &patch_horizontal, input_params);
            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: pixel_value });
        }
        patch_horizontal.clear();  // in a new line there are new values
    }
}

fn compute_gaussian_kernel(radius: i32) -> Kernel1D {
    let mut kernel: Vec<i32> = Vec::new();
    let mut weights_sum = 0;
    let exponent_factor = -1.0 / (2.0 * (radius * radius) as f64);

    for x in -radius..=radius {
        let weight;

        if x <= 0 {  // to avoid computing same values twice for a symmetric function
            weight = E.powf((x * x) as f64 * exponent_factor).round() as i32;
        } else {
            weight = kernel[x as usize];
        }

        kernel.push(weight);
        weights_sum += weight;
    }

    return Kernel1D::new(kernel, weights_sum);
}

fn set_pixel_values(pixel_value: &mut [u8; 4], original_pixel_value: &[u8; 4], patch: &Patch1D, input_params: &LinearInput) {
    let new_value_red = if input_params.channels.red { patch.weighted_average_red() } else { original_pixel_value[0] };
    let new_value_green = if input_params.channels.green { patch.weighted_average_green() } else { original_pixel_value[1] };
    let new_value_blue = if input_params.channels.green { patch.weighted_average_blue() } else { original_pixel_value[2] };
    let new_value_alpha = if input_params.channels.alpha { patch.weighted_average_alpha() } else { original_pixel_value[3] };

    pixel_value[0] = new_value_red;
    pixel_value[1] = new_value_green;
    pixel_value[2] = new_value_blue;
    pixel_value[3] = new_value_alpha;
}

fn blur_vertically(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &LinearInput) {
    let dimensions = image.dimensions();
    let kernel = compute_gaussian_kernel(input_params.radius_vertical as i32);
    let mut patch_vertical = Patch1D::new(input_params.radius_vertical, Some(kernel));

    for u in 0..dimensions.0 {
        for v in 0..dimensions.1 {
            let mut pixel_value = empty_image.get_pixel(u as u32, v as u32).0;
            let original_pixel_value = image.get_pixel(u as u32, v as u32).0;

            _linear_filter::fill_patch_vertically(&image, &mut patch_vertical, input_params, u, v, dimensions.1);
            set_pixel_values(&mut pixel_value, &original_pixel_value, &patch_vertical, input_params);
            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: pixel_value });
        }
        patch_vertical.clear();  // in a new line there are new values
    }
}