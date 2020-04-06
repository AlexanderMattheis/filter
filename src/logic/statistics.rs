use image::{DynamicImage, GenericImageView};

use crate::system::data::composed::statistics_input::StatisticsInput;
use crate::system::data::composed::statistics_output::{StatisticsHistogramOutput, StatisticsOutput};
use crate::system::data::elementary::channels::Channel;
use crate::system::defaults::algorithm_params::{LUMINANCE_BLUE, LUMINANCE_GREEN, LUMINANCE_RED, NUMBER_OF_HISTOGRAM_BINS, NUMBER_OF_HISTOGRAM_CHANNELS};
use crate::system::defaults::messages::errors::{HISTOGRAM_IS_EMPTY, NOT_COMPUTABLE_MEDIAN, print_error_and_quit};

struct HistogramDistribution {
    average: f64,
    variance: f64,
}

pub fn run(image: &DynamicImage, input_params: &StatisticsInput, output_data: &mut [StatisticsOutput; NUMBER_OF_HISTOGRAM_CHANNELS]) {
    let mut statistics_histogram_output = StatisticsHistogramOutput::new();
    compute_histograms(image, &mut statistics_histogram_output);
    let count_pixels = image.pixels().count() as f64;

    if input_params.channels.red {
        compute_statistics(&statistics_histogram_output.red_data, output_data, Channel::RED, count_pixels);
    }

    if input_params.channels.green {
        compute_statistics(&statistics_histogram_output.green_data, output_data, Channel::GREEN, count_pixels);
    }

    if input_params.channels.blue {
        compute_statistics(&statistics_histogram_output.blue_data, output_data, Channel::BLUE, count_pixels);
    }

    if input_params.channels.alpha {
        compute_statistics(&statistics_histogram_output.alpha_data, output_data, Channel::ALPHA, count_pixels);
    }

    if input_params.channels.luminance {
        compute_statistics(&statistics_histogram_output.luminance_data, output_data, Channel::LUMINANCE, count_pixels);
    }
}

fn compute_statistics(histogram: &[u32; NUMBER_OF_HISTOGRAM_BINS],
                      output_data: &mut [StatisticsOutput; NUMBER_OF_HISTOGRAM_CHANNELS],
                      channel: Channel, count_pixels: f64) {
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

fn compute_histograms(image: &DynamicImage, output_data: &mut StatisticsHistogramOutput) {
    let dimensions = image.dimensions();

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            let pixel_value = image.get_pixel(u, v).0;
            let red_value = pixel_value[0] as usize;
            let green_value = pixel_value[1] as usize;
            let blue_value = pixel_value[2] as usize;
            let alpha_value = pixel_value[3] as usize;
            let luminance_value = ((red_value as f64) * LUMINANCE_RED + (green_value as f64) * LUMINANCE_GREEN + (blue_value as f64) * LUMINANCE_BLUE) as usize;

            output_data.red_data[red_value] += 1;
            output_data.green_data[green_value] += 1;
            output_data.blue_data[blue_value] += 1;
            output_data.alpha_data[alpha_value] += 1;
            output_data.luminance_data[luminance_value] += 1;
        }
    }
}

fn get_lowest_pixel_value(histogram: &[u32; NUMBER_OF_HISTOGRAM_BINS]) -> u8 {
    return get_first_non_zero(histogram, (0..NUMBER_OF_HISTOGRAM_BINS as u32).collect());
}

fn get_first_non_zero(histogram: &[u32; NUMBER_OF_HISTOGRAM_BINS], range: Vec<u32>) -> u8 {
    for i in range {
        if histogram[i as usize] > 0 {
            return i as u8;
        }
    }

    print_error_and_quit(HISTOGRAM_IS_EMPTY, None);
}

fn get_highest_pixel_value(histogram: &[u32; NUMBER_OF_HISTOGRAM_BINS]) -> u8 {
    return get_first_non_zero(histogram, (0..NUMBER_OF_HISTOGRAM_BINS as u32).rev().collect());
}

fn get_distribution_parameters(histogram: &[u32; NUMBER_OF_HISTOGRAM_BINS], num_pixels: f64) -> HistogramDistribution {
    let mut sum_a = 0;
    let mut sum_b = 0;

    for i in 0..NUMBER_OF_HISTOGRAM_BINS {
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

fn get_median_pixel_value(histogram: &[u32; NUMBER_OF_HISTOGRAM_BINS], num_pixels: f64) -> u8 {
    let mut h = histogram.clone();

    if (h[0] as f64) >= num_pixels / 2.0 {
        return 0;
    }

    for i in 1..NUMBER_OF_HISTOGRAM_BINS {
        h[i] += h[i - 1];

        if (h[i] as f64) >= num_pixels / 2.0 {
            return i as u8;
        }
    }

    print_error_and_quit(NOT_COMPUTABLE_MEDIAN, None);
}

fn get_dynamics(histogram: &[u32; NUMBER_OF_HISTOGRAM_BINS]) -> u16 {
    let mut count: u16 = 0;

    for i in 0..NUMBER_OF_HISTOGRAM_BINS {
        // each bin stands for a pixel value, and pixel values that have occurred are counted
        if histogram[i] > 0 {
            count += 1;
        }
    }

    return count;
}
