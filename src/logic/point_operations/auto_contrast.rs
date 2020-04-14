use image::DynamicImage;

use crate::logic::_basic_operations;
use crate::system::data::composed::point_operations::auto_contrast_input::AutoContrastInput;
use crate::system::data::composed::statistics_output::StatisticsHistogramOutput;
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;

pub fn run(image: &mut DynamicImage, input_params: &AutoContrastInput) {
    let mut statistics_histogram_output = StatisticsHistogramOutput::new();
    _basic_operations::compute_histograms(image, &mut statistics_histogram_output);

    compute_bounds(&statistics_histogram_output);

    // apply table
    let mut lookup_table: [u8; NUMBER_OF_COLOR_VALUES] = [0; NUMBER_OF_COLOR_VALUES];
    create_lookup_table(input_params, &mut lookup_table);
    _basic_operations::apply_color_lookup_table(image, &lookup_table, &input_params.channels);
}

fn compute_bounds(statistics_histogram_output: &StatisticsHistogramOutput) {}

fn create_lookup_table(input_params: &AutoContrastInput, lookup_table: &mut [u8; NUMBER_OF_COLOR_VALUES]) {
    for i in 0..NUMBER_OF_COLOR_VALUES {}
}