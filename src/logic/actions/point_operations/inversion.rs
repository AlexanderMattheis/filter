use image::DynamicImage;

use crate::logic::algorithm_params::NUM_OF_VALUES;
use crate::logic::data_structures::lookup_table::RgbaLookupTable;
use crate::system::data::composed::point_operations::inversion_input::InversionInput;

pub fn run(image: &mut DynamicImage, input_params: &InversionInput) {
    let mut lookup_table: [u8; NUM_OF_VALUES] = [0; NUM_OF_VALUES];
    create_lookup_table(&mut lookup_table);
    RgbaLookupTable::apply_lookup_table(image, &lookup_table, &input_params.channels);
}

fn create_lookup_table(lookup_table: &mut [u8; NUM_OF_VALUES]) {
    let maximum_value = NUM_OF_VALUES - 1;

    for i in 0..NUM_OF_VALUES {
        lookup_table[i] = (maximum_value - i) as u8;
    }
}