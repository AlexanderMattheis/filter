use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::system::data::composed::point_operations::brightness_input::BrightnessInput;
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;

pub fn run(image: &mut DynamicImage, input_params: &BrightnessInput) {
    let mut lookup_table: [u8; NUMBER_OF_COLOR_VALUES] = [0; NUMBER_OF_COLOR_VALUES];
    create_lookup_table(input_params, &mut lookup_table);

    let dimensions = image.dimensions();
    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            let mut pixel_value = image.get_pixel(u, v).0;

            if input_params.channels.red {
                pixel_value[0] = lookup_table[pixel_value[0] as usize];
            }

            if input_params.channels.green {
                pixel_value[1] = lookup_table[pixel_value[1] as usize];
            }

            if input_params.channels.blue {
                pixel_value[2] = lookup_table[pixel_value[2] as usize];
            }

            if input_params.channels.alpha {
                pixel_value[3] = lookup_table[pixel_value[3] as usize];
            }

            image.put_pixel(u, v, Rgba { 0: pixel_value });
        }
    }
}

fn create_lookup_table(input_params: &BrightnessInput, lookup_table: &mut [u8; NUMBER_OF_COLOR_VALUES]) {
    for i in 0..NUMBER_OF_COLOR_VALUES {
        let new_value = (i as i16) + input_params.value;

        if new_value < 0 {
            lookup_table[i] = 0;
        } else if new_value > (NUMBER_OF_COLOR_VALUES - 1) as i16 {
            lookup_table[i] = 255;
        } else {
            lookup_table[i] = new_value as u8;
        }
    }
}
