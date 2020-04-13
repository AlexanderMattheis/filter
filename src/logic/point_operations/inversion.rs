use image::DynamicImage;

use crate::logic::point_operations::_basic_operations;
use crate::system::data::composed::point_operations::inversion_input::InversionInput;
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;

pub fn run(image: &mut DynamicImage, input_params: &InversionInput) {
    let mut lookup_table: [u8; NUMBER_OF_COLOR_VALUES] = [0; NUMBER_OF_COLOR_VALUES];
    create_lookup_table(&mut lookup_table);
    _basic_operations::apply_lookup_table(image, &lookup_table, &input_params.channels);
}

fn create_lookup_table(lookup_table: &mut [u8; NUMBER_OF_COLOR_VALUES]) {
    let maximum_value = NUMBER_OF_COLOR_VALUES - 1;

    for i in 0..NUMBER_OF_COLOR_VALUES {
        lookup_table[i] = (maximum_value - i) as u8;
    }
}