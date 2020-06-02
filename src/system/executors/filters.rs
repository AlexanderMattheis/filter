use image::{DynamicImage, GenericImageView};

use crate::logic::actions::filters::linear::{box_blur, gaussian_blur};
use crate::logic::actions::filters::non_linear::{median_grayscale_filter, min_max_filter};
use crate::system::defaults::messages::errors;
use crate::system::io::input::filters::linear::{box_blur_parser, gaussian_blur_parser};
use crate::system::io::input::filters::non_linear::{median_filter_parser, min_max_filter_parser};

// linear
pub fn compute_box_blur(image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let mut temp_image_1 = DynamicImage::new_rgba8(image.width(), image.height());
    let mut temp_image_2 = DynamicImage::new_rgba8(image.width(), image.height());
    let input_params = box_blur_parser::parse_params(params, &image.dimensions());

    box_blur::run(image, &mut temp_image_1, &mut temp_image_2, &input_params);
    match temp_image_2.save(output_file_name_path) {
        Err(error) => errors::print_error_and_quit(errors::FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_gaussian_blur(image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let mut temp_image_1 = DynamicImage::new_rgba8(image.width(), image.height());
    let mut temp_image_2 = DynamicImage::new_rgba8(image.width(), image.height());
    let input_params = gaussian_blur_parser::parse_params(params, &image.dimensions());

    gaussian_blur::run(image, &mut temp_image_1, &mut temp_image_2, &input_params);
    match temp_image_2.save(output_file_name_path) {
        Err(error) => errors::print_error_and_quit(errors::FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

// non-linear
pub fn compute_maximum_filter(image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let mut temp_image = DynamicImage::new_rgba8(image.width(), image.height());
    let input_params = min_max_filter_parser::parse_params(params, &image.dimensions());

    min_max_filter::run(image, &mut temp_image, &input_params, false);
    match temp_image.save(output_file_name_path) {
        Err(error) => errors::print_error_and_quit(errors::FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_median_filter(image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let mut temp_image = DynamicImage::new_rgba8(image.width(), image.height());
    let input_params = median_filter_parser::parse_params(params, &image.dimensions());

    median_grayscale_filter::run(image, &mut temp_image, &input_params);
    match temp_image.save(output_file_name_path) {
        Err(error) => errors::print_error_and_quit(errors::FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_minimum_filter(image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let mut temp_image = DynamicImage::new_rgba8(image.width(), image.height());
    let input_params = min_max_filter_parser::parse_params(params, &image.dimensions());

    min_max_filter::run(image, &mut temp_image, &input_params, true);
    match temp_image.save(output_file_name_path) {
        Err(error) => errors::print_error_and_quit(errors::FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}
