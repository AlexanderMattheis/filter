use image::DynamicImage;

use crate::logic::point_operations::{auto_contrast, brightness, contrast, histogram_equalization, histogram_specification, inversion, threshold};
use crate::system::io::input::point_operations::{auto_contrast_parser, brightness_parser, contrast_parser, histogram_equalization_parser, histogram_specification_parser, inversion_parser, threshold_parser};
use crate::logic::histogram;
use crate::system::data::composed::histogram_output::HistogramOutput;
use crate::system::data::composed::histogram_input::HistogramInput;
use crate::system::data::elementary::channels_input::ChannelsInput;

pub fn compute_auto_contrast(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = auto_contrast_parser::parse_params(params);
    auto_contrast::run(image, &input_params);
    image.save(output_file_name_path);
}

pub fn compute_brightness(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = brightness_parser::parse_params(params);
    brightness::run(image, &input_params);
    image.save(output_file_name_path);
}

pub fn compute_contrast(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = contrast_parser::parse_params(params);
    contrast::run(image, &input_params);
    image.save(output_file_name_path);
}

pub fn compute_gamma(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

pub fn compute_histogram_equalization(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = histogram_equalization_parser::parse_params(params);
    histogram_equalization::run(image, &input_params);
    image.save(output_file_name_path);
}

pub fn compute_histogram_specification(image: &mut DynamicImage, reference_image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = histogram_specification_parser::parse_params(params);
    histogram_specification::run(image, reference_image, &input_params);
    image.save(output_file_name_path);
}

pub fn compute_inversion(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = inversion_parser::parse_params(params);
    inversion::run(image, &input_params);
    image.save(output_file_name_path);
}

pub fn compute_linear_blending(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

pub fn compute_threshold(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = threshold_parser::parse_params(params);
    threshold::run(image, &input_params);
    image.save(output_file_name_path);
}