use image::{DynamicImage, GenericImageView, GenericImage, Rgba};
use crate::system::defaults::algorithm_params::{NUMBER_OF_COLOR_VALUES, LUMINANCE_RED, LUMINANCE_GREEN, LUMINANCE_BLUE};
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::composed::statistics_output::StatisticsHistogramOutput;

pub fn apply_color_lookup_table(image: &mut DynamicImage, lookup_table: &[u8; NUMBER_OF_COLOR_VALUES], channels: &RgbaChannelsInput) {
    let dimensions = image.dimensions();
    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            let mut pixel_value = image.get_pixel(u, v).0;

            if channels.red {
                pixel_value[0] = lookup_table[pixel_value[0] as usize];
            }

            if channels.green {
                pixel_value[1] = lookup_table[pixel_value[1] as usize];
            }

            if channels.blue {
                pixel_value[2] = lookup_table[pixel_value[2] as usize];
            }

            if channels.alpha {
                pixel_value[3] = lookup_table[pixel_value[3] as usize];
            }

            image.put_pixel(u, v, Rgba { 0: pixel_value });
        }
    }
}

pub fn apply_average_lookup_table(image: &mut DynamicImage, lookup_table: &[u8; NUMBER_OF_COLOR_VALUES]) {
    let dimensions = image.dimensions();

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            let mut pixel_value = image.get_pixel(u, v).0;

            let average_value = ((pixel_value[0] as f64) * 0.3333
                + (pixel_value[1] as f64) * 0.3333
                + (pixel_value[2] as f64) * 0.3333).round() as usize;

            let new_value = lookup_table[average_value];

            pixel_value[0] = new_value;
            pixel_value[1] = new_value;
            pixel_value[2] = new_value;

            image.put_pixel(u, v, Rgba { 0: pixel_value });
        }
    }
}

pub fn compute_histograms(image: &DynamicImage, output_data: &mut StatisticsHistogramOutput) {
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