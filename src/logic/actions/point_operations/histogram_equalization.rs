use image::DynamicImage;

use crate::logic::algorithm_params::NUM_OF_VALUES;
use crate::logic::data_structures::histogram;
use crate::logic::data_structures::histogram::IntegerRgbaHistogram;
use crate::logic::data_structures::lookup_table::RgbaLookupTable;
use crate::system::data::composed::point_operations::histogram_equalization_input::HistogramEqualizationInput;

pub fn run(image: &mut DynamicImage, input_params: &HistogramEqualizationInput) {
    let mut statistics_histogram_output = IntegerRgbaHistogram::new();
    histogram::compute_cumulative_histograms(image, &mut statistics_histogram_output, input_params.accumulate_roots);

    // apply table
    if input_params.per_channel {
        let mut lookup_tables: RgbaLookupTable = RgbaLookupTable::new();
        create_lookup_tables(input_params, &mut lookup_tables, &statistics_histogram_output);
        RgbaLookupTable::apply_lookup_tables(image, &lookup_tables, &input_params.channels);
    } else {
        let mut averaged_histogram: [u32; NUM_OF_VALUES] = [0; NUM_OF_VALUES];
        histogram::compute_integer_averaged_histogram(&statistics_histogram_output, &mut averaged_histogram, &input_params.channels);

        let mut lookup_table: [u8; NUM_OF_VALUES] = [0; NUM_OF_VALUES];
        create_lookup_table(&mut lookup_table, &averaged_histogram);
        RgbaLookupTable::apply_lookup_table(image, &lookup_table, &input_params.channels);
    }
}

fn create_lookup_tables(input_params: &HistogramEqualizationInput, lookup_tables: &mut RgbaLookupTable, cumulative_histograms: &IntegerRgbaHistogram) {
    if input_params.channels.red {
        create_lookup_table(&mut lookup_tables.red_bound, &cumulative_histograms.red_data);
    }

    if input_params.channels.green {
        create_lookup_table(&mut lookup_tables.green_bound, &cumulative_histograms.green_data);
    }

    if input_params.channels.blue {
        create_lookup_table(&mut lookup_tables.blue_bound, &cumulative_histograms.blue_data);
    }

    if input_params.channels.alpha {
        create_lookup_table(&mut lookup_tables.alpha_bound, &cumulative_histograms.alpha_data);
    }
}

fn create_lookup_table(lookup_table: &mut [u8; NUM_OF_VALUES], cumulative_histogram: &[u32; NUM_OF_VALUES]) {
    let maximum_value = NUM_OF_VALUES - 1;

    let factor = maximum_value as f64 / (cumulative_histogram[maximum_value] as f64);  // cumulative_histogram[maximum_value] = #pixels

    for i in 0..NUM_OF_VALUES {
        lookup_table[i] = (cumulative_histogram[i] as f64 * factor) as u8;
    }
}