use std::path::Path;

use clap::ArgMatches;
use image::DynamicImage;

use crate::logic::actions::{histogram, statistics};
use crate::logic::algorithm_params::NUMBER_OF_INPUT_CHANNELS;
use crate::system::{argument_extractor, manual};
use crate::system::data::composed::histogram_output::HistogramOutput;
use crate::system::data::composed::statistics_output::StatisticsOutput;
use crate::system::defaults::cli::actions;
use crate::system::defaults::messages::errors;
use crate::system::defaults::output_filenames;
use crate::system::executors::{filters, point_operations};
use crate::system::io::input::{histogram_parser, statistics_parser};
use crate::system::io::output::{histogram_builder, statistics_builder};

pub fn execute(matches: &ArgMatches) {
    let arguments = argument_extractor::extract(matches);
    manual::check(&arguments.manual);

    let mut image = match image::open(&arguments.input_path) {
        Ok(image) => image,
        Err(error) => errors::print_error_and_quit(errors::FAILED_LOADING_IMAGE, Some(error.to_string().as_str()))
    };

    if !Path::new(&arguments.output_path).exists() {
        errors::print_error_and_quit(errors::NOT_VALID_PATH, Some(&arguments.output_path));
    }

    let output_file_path_prefix = output_filenames::create_prefix(&arguments.input_path, &arguments.output_path);
    let output_file_name_path = output_filenames::create_output_filepath(&arguments.input_path, &arguments.output_path);

    match arguments.filter.as_str() {
        actions::HISTOGRAM => create_histogram(&image, &arguments.params, &output_file_path_prefix),
        actions::STATISTICS => create_statistics(&image, &arguments.params, &output_file_path_prefix),

        // point operations
        actions::AUTO_CONTRAST => point_operations::compute_auto_contrast(&mut image, &arguments.params, &output_file_name_path),
        actions::BRIGHTNESS => point_operations::compute_brightness(&mut image, &arguments.params, &output_file_name_path),
        actions::CONTRAST => point_operations::compute_contrast(&mut image, &arguments.params, &output_file_name_path),
        actions::GAMMA => point_operations::compute_gamma(&mut image, &arguments.params, &output_file_name_path),
        actions::HISTOGRAM_EQUALIZATION => point_operations::compute_histogram_equalization(&mut image, &arguments.params, &output_file_name_path),
        actions::HISTOGRAM_SPECIFICATION => {
            let reference_image = match image::open(&arguments.reference_path) {
                Ok(image) => image,
                Err(error) => errors::print_error_and_quit(errors::FAILED_LOADING_REFERENCE_IMAGE, Some(error.to_string().as_str()))
            };
            point_operations::compute_histogram_specification(&mut image, &reference_image, &arguments.params, &output_file_name_path);
        }
        actions::INVERSION => point_operations::compute_inversion(&mut image, &arguments.params, &output_file_name_path),
        actions::LINEAR_BLENDING => {
            let reference_image = match image::open(&arguments.reference_path) {
                Ok(image) => image,
                Err(error) => errors::print_error_and_quit(errors::FAILED_LOADING_REFERENCE_IMAGE, Some(error.to_string().as_str()))
            };
            point_operations::compute_linear_blending(&mut image, &reference_image, &arguments.params, &output_file_name_path)
        }
        actions::THRESHOLD => point_operations::compute_threshold(&mut image, &arguments.params, &output_file_name_path),

        // filters
        actions::BOX_BLUR => filters::compute_box_blur(&image, &arguments.params, &output_file_name_path),
        actions::GAUSSIAN_BLUR => filters::compute_gaussian_blur(&image, &arguments.params, &output_file_name_path),

        // _
        _ => errors::print_error_and_quit(errors::NOT_VALID_FILTER, Some(arguments.filter.as_str())),
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