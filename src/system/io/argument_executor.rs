use std::path::Path;

use clap::ArgMatches;
use image::DynamicImage;

use crate::logic::histogram;
use crate::system::defaults::algorithm_params::NUMBER_OF_HISTOGRAM_BINS;
use crate::system::defaults::cli::filters;
use crate::system::defaults::messages::errors;
use crate::system::defaults::messages::errors::print_error_and_quit;
use crate::system::defaults::output_filenames;
use crate::system::io::argument_extractor;
use crate::system::io::data::composed::histogram_output::HistogramOutput;
use crate::system::io::parser::histogram_parser;
use crate::system::io::plotter::histogram_creator;

pub fn execute(matches: &ArgMatches) {
    let arguments = argument_extractor::extract(matches);

    let loaded_image = image::open(&arguments.input_path).unwrap();

    if !Path::new(&arguments.input_path).exists() || !Path::new(&arguments.output_path).exists() {
        print_error_and_quit(errors::NOT_EXISTENT_PATH, &arguments.output_path);
    }

    let output_filepath_prefix = output_filenames::create_prefix(&arguments.input_path, &arguments.output_path);

    match arguments.filter.as_str() {
        filters::HISTOGRAM => create_histogram(&loaded_image, &arguments.params, &output_filepath_prefix),
        _ => errors::print_error_and_quit(errors::NOT_EXISTENT_FILTER, arguments.filter.as_str())
    }
}

fn create_histogram(image: &DynamicImage, params: &String, output_filepath_prefix: &String) {
    let input_params = histogram_parser::parse_params(params);

    let mut histogram_output: HistogramOutput = HistogramOutput {
        red_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
        green_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
        blue_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
        alpha_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
        luminance_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
    };

    histogram::run(image, &input_params, &mut histogram_output);
    histogram_creator::create_histograms(&input_params, &histogram_output, &output_filepath_prefix);
}

