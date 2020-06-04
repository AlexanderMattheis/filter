use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::logic::actions::filters::border_handling::BorderHandling;
use crate::logic::actions::filters::non_linear::_non_linear_filter;
use crate::logic::data_structures::patches::median_stack_patch::MedianStackPatch;
use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::non_linear::median_filter_input::MedianFilterInput;

pub fn run(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &MedianFilterInput) {
    compute_median(image, empty_image, &input_params.filter_input);
}

fn compute_median(image: &DynamicImage, empty_image: &mut DynamicImage, filter_input: &FilterInput) {
    let dim = image.dimensions();
    let dimensions = (dim.0 as i32, dim.1 as i32);

    let border_handling = BorderHandling::new(&filter_input.border_handling);
    let mut median_patch = MedianStackPatch::new(filter_input.radius_horizontal, filter_input.radius_vertical);

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            fill(image, &border_handling, &(u, v), &dimensions,
                 &mut median_patch, &filter_input);
            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: median_patch.median_rgba() });
        }
        median_patch.clear();  // in a new line there are new values
    }
}

fn fill(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
        dimensions: &(i32, i32), median_patch: &mut MedianStackPatch, filter_input: &FilterInput) {
    if position.0 == 0 {
        init_first_position(image, &border_handling, position, &dimensions, median_patch, filter_input);
    } else {
        set_patch(image, &border_handling, position, &dimensions, median_patch, filter_input);
    }
}

fn init_first_position(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                       dimensions: &(i32, i32), median_patch: &mut MedianStackPatch, filter_params: &FilterInput) {
    let vertical = filter_params.radius_vertical as i32;
    let horizontal = filter_params.radius_horizontal as i32;

    for i in -vertical..=vertical {
        for j in -horizontal..=horizontal {
            let pixel_value = _non_linear_filter::get_pixel_value(image, border_handling, i, j, position, dimensions, filter_params);
            median_patch.insert_rgba_at_back(&pixel_value);
        }
    }
}

pub fn set_patch(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                 dimensions: &(i32, i32), median_patch: &mut MedianStackPatch, filter_params: &FilterInput) {
    let horizontal = filter_params.radius_horizontal as i32;
    let vertical = filter_params.radius_vertical as i32;

    for i in -vertical..=vertical {
        let new_pixel_value = _non_linear_filter::get_pixel_value(image, border_handling, i, horizontal, position, dimensions, filter_params);
        median_patch.insert_rgba_at_back(&new_pixel_value);
    }
}
