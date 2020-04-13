use image::{DynamicImage, GenericImageView};

use crate::system::defaults::algorithm_params::LUMINANCE_BLUE;
use crate::system::defaults::algorithm_params::LUMINANCE_GREEN;
use crate::system::defaults::algorithm_params::LUMINANCE_RED;
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;
use crate::system::data::composed::{{histogram_input::HistogramInput, histogram_output::HistogramOutput}};

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

    if input_params.cumulative {
        compute_cumulative(output_data);
    }

    if input_params.logarithmic {
        compute_logarithms(output_data);
    }
}

fn compute_cumulative(output_data: &mut HistogramOutput) {
    for i in 1..NUMBER_OF_COLOR_VALUES {
        // computing cumulative histograms out of non-cumulative histograms
        output_data.red_data[i] += output_data.red_data[i - 1];
        output_data.green_data[i] += output_data.green_data[i - 1];
        output_data.blue_data[i] += output_data.blue_data[i - 1];
        output_data.alpha_data[i] += output_data.alpha_data[i - 1];
        output_data.luminance_data[i] += output_data.luminance_data[i - 1];
    }
}

fn compute_logarithms(output_data: &mut HistogramOutput) {
    for i in 0..NUMBER_OF_COLOR_VALUES {
        // HINT: f64::log10(0.0) = -inf, that is why it works
        output_data.red_data[i] = f64::log10(output_data.red_data[i]);
        output_data.green_data[i] = f64::log10(output_data.green_data[i]);
        output_data.blue_data[i] = f64::log10(output_data.blue_data[i]);
        output_data.alpha_data[i] = f64::log10(output_data.alpha_data[i]);
        output_data.luminance_data[i] = f64::log10(output_data.luminance_data[i]);
    }
}