use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::logic::algorithm_params::{AVERAGE_RGB_VALUE, NUMBER_OF_COLOR_VALUES};
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::defaults::messages::errors;

pub struct RgbaLookupTable {
    pub red_bound: [u8; NUMBER_OF_COLOR_VALUES],
    pub green_bound: [u8; NUMBER_OF_COLOR_VALUES],
    pub blue_bound: [u8; NUMBER_OF_COLOR_VALUES],
    pub alpha_bound: [u8; NUMBER_OF_COLOR_VALUES],
}

impl RgbaLookupTable {
    pub fn new() -> RgbaLookupTable {
        return RgbaLookupTable {
            red_bound: [0; 256],
            green_bound: [0; 256],
            blue_bound: [0; 256],
            alpha_bound: [0; 256],
        };
    }

    pub fn apply_lookup_table(image: &mut DynamicImage, lookup_table: &[u8; NUMBER_OF_COLOR_VALUES], channels: &RgbaChannelsInput) {
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

    pub fn apply_2d_lookup_table(image: &mut DynamicImage, ref_image: &DynamicImage, lookup_table: &[[u8; NUMBER_OF_COLOR_VALUES]; NUMBER_OF_COLOR_VALUES], channels: &RgbaChannelsInput) {
        let image_dimensions = image.dimensions();
        let ref_image_dimensions = ref_image.dimensions();

        if image_dimensions.0 != ref_image_dimensions.0 || image_dimensions.1 != ref_image_dimensions.1 {
            errors::print_error_and_quit(errors::NOT_SAME_SIZE_PICTURES, None);
        }

        for v in 0..image_dimensions.1 {
            for u in 0..image_dimensions.0 {
                let mut pixel_value = image.get_pixel(u, v).0;
                let ref_pixel_value = ref_image.get_pixel(u, v).0;

                if channels.red {
                    pixel_value[0] = lookup_table[pixel_value[0] as usize][ref_pixel_value[0] as usize];
                }

                if channels.green {
                    pixel_value[1] = lookup_table[pixel_value[1] as usize][ref_pixel_value[1] as usize];
                }

                if channels.blue {
                    pixel_value[2] = lookup_table[pixel_value[2] as usize][ref_pixel_value[2] as usize];
                }

                if channels.alpha {
                    pixel_value[3] = lookup_table[pixel_value[3] as usize][ref_pixel_value[3] as usize];
                }

                image.put_pixel(u, v, Rgba { 0: pixel_value });
            }
        }
    }

    pub fn apply_lookup_tables(image: &mut DynamicImage, lookup_tables: &RgbaLookupTable, channels: &RgbaChannelsInput) {
        let dimensions = image.dimensions();

        for v in 0..dimensions.1 {
            for u in 0..dimensions.0 {
                let mut pixel_value = image.get_pixel(u, v).0;

                if channels.red {
                    pixel_value[0] = lookup_tables.red_bound[pixel_value[0] as usize];
                }

                if channels.green {
                    pixel_value[1] = lookup_tables.green_bound[pixel_value[1] as usize];
                }

                if channels.blue {
                    pixel_value[2] = lookup_tables.blue_bound[pixel_value[2] as usize];
                }

                if channels.alpha {
                    pixel_value[3] = lookup_tables.alpha_bound[pixel_value[3] as usize];
                }

                image.put_pixel(u, v, Rgba { 0: pixel_value });
            }
        }
    }

    pub fn apply_average_lookup_table(image: &mut DynamicImage, lookup_table: &[u8; NUMBER_OF_COLOR_VALUES]) {
        apply_lookup_table_weighted(image, lookup_table, AVERAGE_RGB_VALUE, AVERAGE_RGB_VALUE, AVERAGE_RGB_VALUE);
    }
}

fn apply_lookup_table_weighted(image: &mut DynamicImage, lookup_table: &[u8; NUMBER_OF_COLOR_VALUES],
                               weight_red: f64, weight_green: f64, weight_blue: f64) {
    let dimensions = image.dimensions();

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            let mut pixel_value = image.get_pixel(u, v).0;

            let average_value = ((pixel_value[0] as f64) * weight_red
                + (pixel_value[1] as f64) * weight_green
                + (pixel_value[2] as f64) * weight_blue).round() as usize;

            let new_value = lookup_table[average_value];

            pixel_value[0] = new_value;
            pixel_value[1] = new_value;
            pixel_value[2] = new_value;

            image.put_pixel(u, v, Rgba { 0: pixel_value });
        }
    }
}