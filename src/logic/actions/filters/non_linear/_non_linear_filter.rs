use image::{DynamicImage, GenericImageView};

use crate::logic::actions::filters::border_handling::BorderHandling;
use crate::logic::algorithm_params::{NUM_OF_VALUES, NUM_OF_VALUES_SUM};
use crate::system::data::composed::filters::filter_input::FilterInput;

pub type LookupTable = [[u16; NUM_OF_VALUES]; NUM_OF_VALUES_SUM];

pub fn create_lookup_table(lookup_table: &mut LookupTable) {
    for i in 0..NUM_OF_VALUES_SUM {
        for j in 0..NUM_OF_VALUES {
            lookup_table[i][j] = (i + j) as u16;
        }
    }
}

pub fn get_pixel_value(image: &DynamicImage, border_handling: &BorderHandling, i: i32, j: i32,
                   position: &(i32, i32), dimensions: &(i32, i32), filter_params: &FilterInput) -> [u8; 4] {
    let pixel_value;

    if position.0 + j < 0 || position.0 + j >= dimensions.0 || position.1 + i < 0 || position.1 + i >= dimensions.1 {
        pixel_value = (border_handling.get_pixel)(image, position.0 + j, position.1 + i, &filter_params.background_color);
    } else {
        pixel_value = image.get_pixel((position.0 + j) as u32, (position.1 + i) as u32).0;
    }

    return pixel_value;
}
