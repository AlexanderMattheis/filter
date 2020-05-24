use image::DynamicImage;

use crate::logic::algorithm_params::NUM_OF_VALUES;
use crate::logic::data_structures::lookup_table::RgbaLookupTable;
use crate::system::data::composed::point_operations::gamma_input::GammaInput;

pub fn run(image: &mut DynamicImage, input_params: &GammaInput) {
    let mut lookup_table: [u8; NUM_OF_VALUES] = [0; NUM_OF_VALUES];
    create_lookup_table(input_params, &mut lookup_table);
    RgbaLookupTable::apply_lookup_table(image, &lookup_table, &input_params.channels);
}

fn create_lookup_table(input_params: &GammaInput, lookup_table: &mut [u8; NUM_OF_VALUES]) {
    let maximum_value = (NUM_OF_VALUES - 1) as f64;

    for i in 0..NUM_OF_VALUES {
        let new_value = (i as f64 / maximum_value).powf(input_params.value) * maximum_value;

        // no clamping necessary, (relative value)^gamma can't get higher than 1.0
        lookup_table[i] = new_value.round() as u8;
    }
}