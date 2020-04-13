use std::path::Path;

use clap::ArgMatches;
use image::DynamicImage;

use crate::logic::{histogram, statistics};
use crate::system::argument_extractor;
use crate::system::data::composed::histogram_output::HistogramOutput;
use crate::system::data::composed::statistics_output::StatisticsOutput;
use crate::system::defaults::algorithm_params::NUMBER_OF_INPUT_CHANNELS;
use crate::system::defaults::cli::filters;
use crate::system::defaults::messages::errors;
use crate::system::defaults::messages::errors::print_error_and_quit;
use crate::system::defaults::output_filenames;
use crate::system::io::input::{histogram_parser, statistics_parser};
use crate::system::io::output::{histogram_builder, statistics_builder};
use crate::system::io::input::point_operations::{brightness_parser, contrast_parser};
use crate::logic::point_operations::{brightness, contrast};
use crate::system::basic::strings;

pub fn execute(matches: &ArgMatches) {
    let arguments = argument_extractor::extract(matches);

    let mut loaded_image = image::open(&arguments.input_path).unwrap();

    if !Path::new(&arguments.input_path).exists() || !Path::new(&arguments.output_path).exists() {
        print_error_and_quit(errors::NOT_VALID_PATH, Some(&arguments.output_path));
    }

    let output_file_path_prefix = output_filenames::create_prefix(&arguments.input_path, &arguments.output_path);
    let output_file_name_path = output_filenames::create_output_filepath(&arguments.input_path, &arguments.output_path);

    match arguments.filter.as_str() {
        filters::HISTOGRAM => create_histogram(&loaded_image, &arguments.params, &output_file_path_prefix),
        filters::STATISTICS => create_statistics(&loaded_image, &arguments.params, &output_file_path_prefix),

        // point operations
        filters::AUTO_CONTRAST => compute_auto_contrast(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::BRIGHTNESS => compute_brightness(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::CONTRAST => compute_contrast(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::GAMMA => compute_gamma(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::HISTOGRAM_EQUALIZATION => compute_histogram_equalization(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::HISTOGRAM_SPECIFICATION => compute_histogram_specification(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::INVERSION => compute_inversion(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::LINEAR_BLENDING => compute_linear_blending(&mut loaded_image, &arguments.params, &output_file_name_path),
        filters::THRESHOLD => compute_threshold(&mut loaded_image, &arguments.params, &output_file_name_path),
        _ => errors::print_error_and_quit(errors::NOT_VALID_FILTER, Some(arguments.filter.as_str()))
    }
}

fn create_histogram(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {
    let input_params = histogram_parser::parse_params(params);

    let mut histogram_output: HistogramOutput = HistogramOutput::new();
    histogram::run(image, &input_params, &mut histogram_output);
    histogram_builder::create_histograms(&input_params, &histogram_output, &output_filepath_prefix);
}

fn create_statistics(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {
    let input_params = statistics_parser::parse_params(params);

    let mut statistics_output: [StatisticsOutput; NUMBER_OF_INPUT_CHANNELS] = StatisticsOutput::new();
    statistics::run(image, &input_params, &mut statistics_output);
    statistics_builder::create_statistics(&input_params, &statistics_output, &output_filepath_prefix);
}

// point operations
fn compute_auto_contrast(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

fn compute_brightness(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = brightness_parser::parse_params(params);
    brightness::run(image, &input_params);
    image.save(output_file_name_path);
}

fn compute_contrast(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {
    let input_params = contrast_parser::parse_params(params);
    contrast::run(image, &input_params);
    image.save(output_file_name_path);
}

fn compute_gamma(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

fn compute_histogram_equalization(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

fn compute_histogram_specification(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

fn compute_inversion(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

fn compute_linear_blending(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}

fn compute_threshold(image: &mut DynamicImage, params: &String, output_file_name_path: &String) {}