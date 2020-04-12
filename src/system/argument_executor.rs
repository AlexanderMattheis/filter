use std::path::Path;

use clap::ArgMatches;
use image::DynamicImage;

use crate::logic::{histogram, statistics};
use crate::system::argument_extractor;
use crate::system::data::composed::histogram_output::HistogramOutput;
use crate::system::data::composed::statistics_output::StatisticsOutput;
use crate::system::defaults::algorithm_params::NUMBER_OF_HISTOGRAM_CHANNELS;
use crate::system::defaults::cli::filters;
use crate::system::defaults::messages::errors;
use crate::system::defaults::messages::errors::print_error_and_quit;
use crate::system::defaults::output_filenames;
use crate::system::io::input::{histogram_parser, statistics_parser};
use crate::system::io::output::{histogram_builder, statistics_builder};

pub fn execute(matches: &ArgMatches) {
    let arguments = argument_extractor::extract(matches);

    let loaded_image = image::open(&arguments.input_path).unwrap();

    if !Path::new(&arguments.input_path).exists() || !Path::new(&arguments.output_path).exists() {
        print_error_and_quit(errors::NOT_EXISTENT_PATH, Some(&arguments.output_path));
    }

    let output_filepath_prefix = output_filenames::create_prefix(&arguments.input_path, &arguments.output_path);

    match arguments.filter.as_str() {
        filters::HISTOGRAM => create_histogram(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::STATISTICS => create_statistics(&loaded_image, &arguments.params, &output_filepath_prefix),

        // point operations
        filters::AUTO_CONTRAST => compute_auto_contrast(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::BRIGHTNESS => compute_brightness(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::CONTRAST => compute_contrast(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::GAMMA => compute_gamma(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::HISTOGRAM_EQUALIZATION => compute_histogram_equalization(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::HISTOGRAM_SPECIFICATION => compute_histogram_specification(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::INVERSION => compute_inversion(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::LINEAR_BLENDING => compute_linear_blending(&loaded_image, &arguments.params, &output_filepath_prefix),
        filters::THRESHOLD => compute_threshold(&loaded_image, &arguments.params, &output_filepath_prefix),
        _ => errors::print_error_and_quit(errors::NOT_EXISTENT_FILTER, Some(arguments.filter.as_str()))
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

    let mut statistics_output: [StatisticsOutput; NUMBER_OF_HISTOGRAM_CHANNELS] = StatisticsOutput::new();
    statistics::run(image, &input_params, &mut statistics_output);
    statistics_builder::create_statistics(&input_params, &statistics_output, &output_filepath_prefix);
}

// point operations
fn compute_auto_contrast(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_brightness(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_contrast(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_gamma(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_histogram_equalization(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_histogram_specification(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_inversion(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_linear_blending(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {}

fn compute_threshold(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {} 