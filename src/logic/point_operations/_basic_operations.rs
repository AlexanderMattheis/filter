use image::{DynamicImage, GenericImageView, GenericImage, Rgba};
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;

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