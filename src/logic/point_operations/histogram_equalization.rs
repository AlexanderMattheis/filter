use image::DynamicImage;

use crate::logic::_basic_operations;
use crate::system::data::bounds::RgbaPixelBounds;
use crate::system::data::composed::point_operations::histogram_equalization_input::HistogramEqualizationInput;
use crate::system::data::composed::statistics_output::StatisticsHistogramOutput;
use crate::system::data::lookup_tables::LookupTables;
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;

pub fn run(image: &mut DynamicImage, input_params: &HistogramEqualizationInput) {
    let mut statistics_histogram_output = StatisticsHistogramOutput::new();
    _basic_operations::compute_cumulative_histograms(image, &mut statistics_histogram_output, input_params.accumulate_roots);

    // apply table
    if input_params.per_channel {
        let mut lookup_tables: LookupTables = LookupTables::new();
        create_lookup_tables(input_params, &mut lookup_tables, &statistics_histogram_output);
        _basic_operations::apply_lookup_tables(image, &lookup_tables, &input_params.channels);
    } else {
        let mut averaged_histogram: [u32; NUMBER_OF_COLOR_VALUES] = [0; NUMBER_OF_COLOR_VALUES];
        _basic_operations::compute_averaged_histogram(&statistics_histogram_output, &mut averaged_histogram, &input_params.channels);

        let mut lookup_table: [u8; NUMBER_OF_COLOR_VALUES] = [0; NUMBER_OF_COLOR_VALUES];
        create_lookup_table(input_params, &mut lookup_table, &averaged_histogram);
        _basic_operations::apply_lookup_table(image, &lookup_table, &input_params.channels);
    }
}

fn create_lookup_tables(input_params: &HistogramEqualizationInput, lookup_tables: &mut LookupTables, cumulative_histograms: &StatisticsHistogramOutput) {
    create_lookup_table(input_params, &mut lookup_tables.red_bound, &cumulative_histograms.red_data);
    create_lookup_table(input_params, &mut lookup_tables.green_bound, &cumulative_histograms.green_data);
    create_lookup_table(input_params, &mut lookup_tables.blue_bound, &cumulative_histograms.blue_data);
    create_lookup_table(input_params, &mut lookup_tables.alpha_bound, &cumulative_histograms.alpha_data);
}

fn create_lookup_table(input_params: &HistogramEqualizationInput, lookup_table: &mut [u8; NUMBER_OF_COLOR_VALUES], cumulative_histogram: &[u32; NUMBER_OF_COLOR_VALUES]) {
    let maximum_value = (NUMBER_OF_COLOR_VALUES - 1);

    let factor = maximum_value as f64 / (cumulative_histogram[maximum_value] as f64);  // cumulative_histogram[maximum_value] = #pixels

    for i in 0..NUMBER_OF_COLOR_VALUES {
        lookup_table[i] = (cumulative_histogram[i] as f64 * factor) as u8;
    }
}