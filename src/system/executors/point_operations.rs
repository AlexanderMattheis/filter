use image::DynamicImage;

use crate::logic::point_operations::{auto_contrast, brightness, contrast, gamma, histogram_equalization, histogram_specification, inversion, linear_blending, threshold};
use crate::system::defaults::messages::errors::{FAILED_SAVING_IMAGE, print_error_and_quit};
use crate::system::io::input::point_operations::{auto_contrast_parser, brightness_parser, contrast_parser, gamma_parser, histogram_equalization_parser, histogram_specification_parser, inversion_parser, linear_blending_parser, threshold_parser};

pub fn compute_auto_contrast(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = auto_contrast_parser::parse_params(params);
    auto_contrast::run(image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_brightness(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = brightness_parser::parse_params(params);
    brightness::run(image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_contrast(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = contrast_parser::parse_params(params);
    contrast::run(image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    };
}

pub fn compute_gamma(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = gamma_parser::parse_params(params);
    gamma::run(image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    };
}

pub fn compute_histogram_equalization(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = histogram_equalization_parser::parse_params(params);
    histogram_equalization::run(image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_histogram_specification(image: &mut DynamicImage, reference_image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = histogram_specification_parser::parse_params(params);
    histogram_specification::run(image, reference_image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_inversion(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = inversion_parser::parse_params(params);
    inversion::run(image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_linear_blending(image: &mut DynamicImage, reference_image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = linear_blending_parser::parse_params(params);
    linear_blending::run(image, reference_image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}

pub fn compute_threshold(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = threshold_parser::parse_params(params);
    threshold::run(image, &input_params);
    match image.save(output_file_name_path) {
        Err(error) => print_error_and_quit(FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}