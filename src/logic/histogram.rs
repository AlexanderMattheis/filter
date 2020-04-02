use image::{DynamicImage, GenericImageView};

use crate::system::defaults::algorithm_params::NUMBER_OF_HISTOGRAM_BINS;
use crate::system::io::data::composed::{histogram_input::HistogramInput, histogram_output::HistogramOutput};

// constants taken from https://gitlab.gnome.org/GNOME/gimp/blob/master/libgimpcolor/gimprgb.h
const LUMINANCE_RED: f64 = 0.22248840;
const LUMINANCE_GREEN: f64 = 0.71690369;
const LUMINANCE_BLUE: f64 = 0.06060791;

pub fn run(image: &DynamicImage, input_params: &HistogramInput, output_data: &mut HistogramOutput) {
    let dimensions = image.dimensions();

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            let pixel_value = image.get_pixel(u, v).0;
            let red_value = pixel_value[0] as usize;
            let green_value = pixel_value[1] as usize;
            let blue_value = pixel_value[2] as usize;
            let alpha_value = pixel_value[3] as usize;
            let luminance_value = ((red_value as f64) * LUMINANCE_RED + (green_value as f64) * LUMINANCE_GREEN + (blue_value as f64) * LUMINANCE_BLUE) as usize;

            output_data.red_data[red_value] += 1.0;
            output_data.green_data[green_value] += 1.0;
            output_data.blue_data[blue_value] += 1.0;
            output_data.alpha_data[alpha_value] += 1.0;
            output_data.luminance_data[luminance_value] += 1.0;
        }
    }

    if input_params.logarithmic {
        compute_logarithms(output_data);
    }
}

fn compute_logarithms(output_data: &mut HistogramOutput) {
    for i in 0..NUMBER_OF_HISTOGRAM_BINS {
        output_data.red_data[i] = f64::log10(output_data.red_data[i]);
        output_data.green_data[i] = f64::log10(output_data.green_data[i]);
        output_data.blue_data[i] = f64::log10(output_data.blue_data[i]);
        output_data.alpha_data[i] = f64::log10(output_data.alpha_data[i]);
        output_data.luminance_data[i] = f64::log10(output_data.luminance_data[i]);
    }
}