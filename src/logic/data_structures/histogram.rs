use image::{DynamicImage, GenericImageView};

use crate::logic::algorithm_params::{LUMINANCE_BLUE, LUMINANCE_GREEN, LUMINANCE_RED, NUM_OF_VALUES};
use crate::system::data::elementary::channels_input::RgbaChannelsInput;

pub struct FloatingPointRgbaHistogram {
    pub red_data: [f64; NUM_OF_VALUES],
    pub green_data: [f64; NUM_OF_VALUES],
    pub blue_data: [f64; NUM_OF_VALUES],
    pub alpha_data: [f64; NUM_OF_VALUES],
}

impl FloatingPointRgbaHistogram {
    pub fn new() -> FloatingPointRgbaHistogram {
        return FloatingPointRgbaHistogram {
            red_data: [0.0; NUM_OF_VALUES],
            green_data: [0.0; NUM_OF_VALUES],
            blue_data: [0.0; NUM_OF_VALUES],
            alpha_data: [0.0; NUM_OF_VALUES],
        };
    }
}

pub struct IntegerRgbaHistogram {
    pub red_data: [u32; NUM_OF_VALUES],
    pub green_data: [u32; NUM_OF_VALUES],
    pub blue_data: [u32; NUM_OF_VALUES],
    pub alpha_data: [u32; NUM_OF_VALUES],
    pub luminance_data: [u32; NUM_OF_VALUES],
}

impl IntegerRgbaHistogram {
    pub fn new() -> IntegerRgbaHistogram {
        return IntegerRgbaHistogram {
            red_data: [0; NUM_OF_VALUES],
            green_data: [0; NUM_OF_VALUES],
            blue_data: [0; NUM_OF_VALUES],
            alpha_data: [0; NUM_OF_VALUES],
            luminance_data: [0; NUM_OF_VALUES],
        };
    }
}

pub fn compute_cumulative_histograms(image: &DynamicImage, output_data: &mut IntegerRgbaHistogram, accumulate_roots: bool) {
    compute_histograms(image, output_data);

    if accumulate_roots {
        integrate_discrete_root(output_data);
    } else {
        integrate_discrete(output_data);
    }
}

pub fn compute_histograms(image: &DynamicImage, output_data: &mut IntegerRgbaHistogram) {
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

pub fn compute_integer_averaged_histogram(histograms: &IntegerRgbaHistogram, averaged_histogram: &mut [u32; 256], channels: &RgbaChannelsInput) {
    let mut count = 0.0;

    if channels.red {
        count += 1.0;
    }

    if channels.green {
        count += 1.0;
    }

    if channels.blue {
        count += 1.0;
    }

    if channels.alpha {
        count += 1.0;
    }

    let divisor_factor = 1.0 / count;

    for i in 0..averaged_histogram.len() {
        averaged_histogram[i] += if channels.red { histograms.red_data[i] } else { 0 };
        averaged_histogram[i] += if channels.green { histograms.green_data[i] } else { 0 };
        averaged_histogram[i] += if channels.blue { histograms.blue_data[i] } else { 0 };
        averaged_histogram[i] += if channels.alpha { histograms.alpha_data[i] } else { 0 };
        averaged_histogram[i] = (averaged_histogram[i] as f64 * divisor_factor).round() as u32;
    }
}

fn integrate_discrete_root(histograms: &mut IntegerRgbaHistogram) {
    let mut histogram_red: [f64; 256] = [0.0; 256];
    let mut histogram_green: [f64; 256] = [0.0; 256];
    let mut histogram_blue: [f64; 256] = [0.0; 256];
    let mut histogram_alpha: [f64; 256] = [0.0; 256];
    let mut histogram_luminance: [f64; 256] = [0.0; 256];

    histogram_red[0] = f64::sqrt(histograms.red_data[0] as f64);
    histogram_green[0] = f64::sqrt(histograms.green_data[0] as f64);
    histogram_blue[0] = f64::sqrt(histograms.blue_data[0] as f64);
    histogram_alpha[0] = f64::sqrt(histograms.alpha_data[0] as f64);
    histogram_luminance[0] = f64::sqrt(histograms.luminance_data[0] as f64);

    // sum up non-rounded square-roots
    for i in 1..NUM_OF_VALUES {
        histogram_red[i] = histogram_red[i - 1] + f64::sqrt(histograms.red_data[i] as f64);
        histogram_green[i] = histogram_green[i - 1] + f64::sqrt(histograms.green_data[i] as f64);
        histogram_blue[i] = histogram_blue[i - 1] + f64::sqrt(histograms.blue_data[i] as f64);
        histogram_alpha[i] = histogram_alpha[i - 1] + f64::sqrt(histograms.alpha_data[i] as f64);
        histogram_luminance[i] = histogram_luminance[i - 1] + f64::sqrt(histograms.luminance_data[i] as f64);
    }

    // store rounded values
    for i in 0..NUM_OF_VALUES {
        histograms.red_data[i] = histogram_red[i].round() as u32;
        histograms.green_data[i] = histogram_green[i].round() as u32;
        histograms.blue_data[i] = histogram_blue[i].round() as u32;
        histograms.alpha_data[i] = histogram_alpha[i].round() as u32;
        histograms.luminance_data[i] = histogram_luminance[i].round() as u32;
    }
}

fn integrate_discrete(histograms: &mut IntegerRgbaHistogram) {
    for i in 1..NUM_OF_VALUES {
        histograms.red_data[i] += histograms.red_data[i - 1];
        histograms.green_data[i] += histograms.green_data[i - 1];
        histograms.blue_data[i] += histograms.blue_data[i - 1];
        histograms.alpha_data[i] += histograms.alpha_data[i - 1];
        histograms.luminance_data[i] += histograms.luminance_data[i - 1];
    }
}