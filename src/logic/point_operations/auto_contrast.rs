use image::{DynamicImage, GenericImageView};

use crate::logic::_basic_operations;
use crate::system::data::bounds::{PixelBound, RgbaPixelBounds};
use crate::system::data::composed::point_operations::auto_contrast_input::AutoContrastInput;
use crate::system::data::composed::statistics_output::StatisticsHistogramOutput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::lookup_tables::LookupTables;
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;
use crate::system::defaults::messages::errors;

pub fn run(image: &mut DynamicImage, input_params: &AutoContrastInput) {
    let mut statistics_histogram_output = StatisticsHistogramOutput::new();
    _basic_operations::compute_cumulative_histograms(image, &mut statistics_histogram_output, false);

    // compute bounds
    let mut rgba_bounds = RgbaPixelBounds::new(255, 0);
    compute_bounds(&mut statistics_histogram_output, &mut rgba_bounds, &input_params, image.dimensions());

    // apply table
    if input_params.per_channel {
        let mut lookup_tables: LookupTables = LookupTables::new();
        create_lookup_tables(input_params, &mut lookup_tables, &rgba_bounds);
        _basic_operations::apply_lookup_tables(image, &lookup_tables, &input_params.channels);
    } else {
        let average_bound = compute_average_bound(&rgba_bounds, &input_params.channels);

        let mut lookup_table: [u8; NUMBER_OF_COLOR_VALUES] = [0; NUMBER_OF_COLOR_VALUES];
        create_lookup_table(input_params, &mut lookup_table, &average_bound);
        _basic_operations::apply_lookup_table(image, &lookup_table, &input_params.channels);
    }
}

fn compute_bounds(statistics_histogram_output: &mut StatisticsHistogramOutput, rgba_bounds: &mut RgbaPixelBounds,
                  input_params: &AutoContrastInput, image_dimensions: (u32, u32)) {
    let number_of_pixels = (image_dimensions.0 * image_dimensions.1) as f64;
    let pixels_bound_lower = (number_of_pixels * input_params.quantile_low).round() as u32;
    let pixels_bound_higher = (number_of_pixels * (1.0 - input_params.quantile_high)).round() as u32;

    rgba_bounds.red_bound = get_bounds(&statistics_histogram_output.red_data, pixels_bound_lower, pixels_bound_higher);
    rgba_bounds.green_bound = get_bounds(&statistics_histogram_output.green_data, pixels_bound_lower, pixels_bound_higher);
    rgba_bounds.blue_bound = get_bounds(&statistics_histogram_output.blue_data, pixels_bound_lower, pixels_bound_higher);
    rgba_bounds.alpha_bound = get_bounds(&statistics_histogram_output.alpha_data, pixels_bound_lower, pixels_bound_higher);
}

fn get_bounds(histogram_data: &[u32; NUMBER_OF_COLOR_VALUES], pixels_bound_lower: u32, pixels_bound_higher: u32) -> PixelBound {
    let lower = get_lower_bound(histogram_data, pixels_bound_lower);
    let higher = get_higher_bound(histogram_data, pixels_bound_higher);

    return PixelBound { lower, higher };
}

fn get_lower_bound(histogram_data: &[u32; NUMBER_OF_COLOR_VALUES], pixels_bound_lower: u32) -> u8 {
    for i in 0..NUMBER_OF_COLOR_VALUES {
        if histogram_data[i] >= pixels_bound_lower {
            return i as u8;
        }
    }

    return 255;
}

fn get_higher_bound(histogram_data: &[u32; NUMBER_OF_COLOR_VALUES], pixels_bound_higher: u32) -> u8 {
    for i in (0..NUMBER_OF_COLOR_VALUES).rev() {
        if histogram_data[i] <= pixels_bound_higher {
            return i as u8;
        }
    }

    return 0;
}

fn compute_average_bound(rgba_bounds: &RgbaPixelBounds, channels: &RgbaChannelsInput) -> PixelBound {
    let mut lower_bound = 0.0;
    let mut higher_bound = 0.0;
    let mut count = 0.0;

    if channels.red {
        lower_bound += rgba_bounds.red_bound.lower as f32;
        higher_bound += rgba_bounds.red_bound.higher as f32;
        count += 1.0;
    }

    if channels.green {
        lower_bound += rgba_bounds.green_bound.lower as f32;
        higher_bound += rgba_bounds.green_bound.higher as f32;
        count += 1.0;
    }

    if channels.blue {
        lower_bound += rgba_bounds.blue_bound.lower as f32;
        higher_bound += rgba_bounds.blue_bound.higher as f32;
        count += 1.0;
    }

    if channels.alpha {
        lower_bound += rgba_bounds.alpha_bound.lower as f32;
        higher_bound += rgba_bounds.alpha_bound.higher as f32;
        count += 1.0;
    }

    let lower = (lower_bound / count).round() as u8;
    let higher = (higher_bound / count).round() as u8;

    return PixelBound { lower, higher };
}

fn create_lookup_tables(input_params: &AutoContrastInput, lookup_tables: &mut LookupTables,
                        rgba_bounds: &RgbaPixelBounds) {
    create_lookup_table(input_params, &mut lookup_tables.red_bound, &rgba_bounds.red_bound);
    create_lookup_table(input_params, &mut lookup_tables.green_bound, &rgba_bounds.green_bound);
    create_lookup_table(input_params, &mut lookup_tables.blue_bound, &rgba_bounds.blue_bound);
    create_lookup_table(input_params, &mut lookup_tables.alpha_bound, &rgba_bounds.alpha_bound);
}

fn create_lookup_table(input_params: &AutoContrastInput, lookup_table: &mut [u8; NUMBER_OF_COLOR_VALUES], pixel_bound: &PixelBound) {
    let maximum_value = (NUMBER_OF_COLOR_VALUES - 1) as u8;

    let low = pixel_bound.lower as usize;
    let high = pixel_bound.higher as usize;

    let contrast_factor = maximum_value as f64 / (high - low) as f64;

    for i in 0..NUMBER_OF_COLOR_VALUES {
        if i <= low {
            lookup_table[i] = 0;
        } else if low < i && i < high {
            lookup_table[i] = ((i - low) as f64 * contrast_factor).round() as u8;
        } else if i >= high {
            lookup_table[i] = maximum_value;
        }
    }
}