use image::{DynamicImage, GenericImageView};

use crate::logic::algorithm_params::{NUMBER_OF_INPUT_CHANNELS, NUM_OF_VALUES};
use crate::logic::data_structures::histogram;
use crate::logic::data_structures::histogram::IntegerRgbaHistogram;
use crate::system::data::composed::statistics_input::StatisticsInput;
use crate::system::data::composed::statistics_output::StatisticsOutput;
use crate::system::defaults::messages::errors;
use crate::system::defaults::types::channel_type::ChannelType;

struct HistogramDistribution {
    average: f64,
    variance: f64,
}

pub fn run(image: &DynamicImage, input_params: &StatisticsInput, output_data: &mut [StatisticsOutput; NUMBER_OF_INPUT_CHANNELS]) {
    let mut rgba_histogram = IntegerRgbaHistogram::new();
    histogram::compute_histograms(image, &mut rgba_histogram);
    let count_pixels = image.pixels().count() as f64;

    if input_params.channels.red {
        compute_statistics(&rgba_histogram.red_data, output_data, ChannelType::Red, count_pixels);
    }

    if input_params.channels.green {
        compute_statistics(&rgba_histogram.green_data, output_data, ChannelType::Green, count_pixels);
    }

    if input_params.channels.blue {
        compute_statistics(&rgba_histogram.blue_data, output_data, ChannelType::Blue, count_pixels);
    }

    if input_params.channels.alpha {
        compute_statistics(&rgba_histogram.alpha_data, output_data, ChannelType::Alpha, count_pixels);
    }

    if input_params.channels.luminance {
        compute_statistics(&rgba_histogram.luminance_data, output_data, ChannelType::Luminance, count_pixels);
    }
}

fn compute_statistics(histogram: &[u32; NUM_OF_VALUES],
                      output_data: &mut [StatisticsOutput; NUMBER_OF_INPUT_CHANNELS],
                      channel: ChannelType, count_pixels: f64) {
    let min = get_lowest_pixel_value(histogram);
    let max = get_highest_pixel_value(histogram);

    let distribution_params = get_distribution_parameters(histogram, count_pixels);
    let average = distribution_params.average;
    let variance = distribution_params.variance;

    let median = get_median_pixel_value(histogram, count_pixels);
    let contrast = max - min;
    let dynamics = get_dynamics(histogram);

    output_data[channel as usize] = StatisticsOutput {
        min,
        max,
        average,
        variance,
        median,
        contrast,
        dynamics,
    }
}

fn get_lowest_pixel_value(histogram: &[u32; NUM_OF_VALUES]) -> u8 {
    return get_first_non_zero(histogram, (0..NUM_OF_VALUES as u32).collect());
}

fn get_first_non_zero(histogram: &[u32; NUM_OF_VALUES], range: Vec<u32>) -> u8 {
    for i in range {
        if histogram[i as usize] > 0 {
            return i as u8;
        }
    }

    errors::print_error_and_quit(errors::NOT_EXISTENT_HISTOGRAM, None);
}

fn get_highest_pixel_value(histogram: &[u32; NUM_OF_VALUES]) -> u8 {
    return get_first_non_zero(histogram, (0..NUM_OF_VALUES as u32).rev().collect());
}

fn get_distribution_parameters(histogram: &[u32; NUM_OF_VALUES], num_pixels: f64) -> HistogramDistribution {
    let mut sum_a = 0;
    let mut sum_b = 0;

    for i in 0..NUM_OF_VALUES {
        let value = (i as u64) * (histogram[i] as u64);

        sum_a += value;
        sum_b += (i as u64) * value;
    }

    let a = sum_a as f64;
    let b = sum_b as f64;
    let count_reciprocal = 1.0 / num_pixels;

    return HistogramDistribution {
        average: count_reciprocal * a,
        variance: (b - count_reciprocal * (a * a)) * count_reciprocal,
    };
}

fn get_median_pixel_value(histogram: &[u32; NUM_OF_VALUES], num_pixels: f64) -> u8 {
    let mut h = histogram.clone();

    if (h[0] as f64) >= num_pixels * 0.5 {
        return 0;
    }

    for i in 1..NUM_OF_VALUES {
        h[i] += h[i - 1];

        if (h[i] as f64) >= num_pixels * 0.5 {
            return i as u8;
        }
    }

    errors::print_error_and_quit(errors::NOT_EXISTENT_MEDIAN, None);
}

fn get_dynamics(histogram: &[u32; NUM_OF_VALUES]) -> u16 {
    let mut count: u16 = 0;

    for i in 0..NUM_OF_VALUES {
        // each bin stands for a pixel value, and pixel values that have occurred are counted
        if histogram[i] > 0 {
            count += 1;
        }
    }

    return count;
}
