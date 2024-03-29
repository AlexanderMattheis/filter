use image::DynamicImage;

use crate::logic::algorithm_params::NUM_OF_VALUES;
use crate::logic::data_structures::lookup_table::RgbaLookupTable;
use crate::system::data::composed::point_operations::linear_blending_input::LinearBlendingInput;

pub fn run(image: &mut DynamicImage, ref_image: &DynamicImage, input_params: &LinearBlendingInput) {
    let mut lookup_table: [[u8; NUM_OF_VALUES]; NUM_OF_VALUES] = [[0; NUM_OF_VALUES]; NUM_OF_VALUES];
    create_lookup_table(input_params, &mut lookup_table);
    RgbaLookupTable::apply_2d_lookup_table(image, ref_image, &lookup_table, &input_params.channels);
}

fn create_lookup_table(input_params: &LinearBlendingInput, lookup_table: &mut [[u8; NUM_OF_VALUES]; NUM_OF_VALUES]) {
    let alpha = input_params.value;
    let one_minus_alpha = 1.0 - alpha;

    for i in 0..NUM_OF_VALUES {
        for j in i..NUM_OF_VALUES {
            lookup_table[i][j] = (alpha * (j as f64) + one_minus_alpha * (i as f64)).round() as u8;
            lookup_table[j][i] = (alpha * (i as f64) + one_minus_alpha * (j as f64)).round() as u8;
        }
    }
}