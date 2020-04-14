use image::DynamicImage;

use crate::logic::_basic_operations;
use crate::system::data::composed::point_operations::threshold_input::ThresholdInput;
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;

pub fn run(image: &mut DynamicImage, input_params: &ThresholdInput) {
    let mut lookup_table: [u8; NUMBER_OF_COLOR_VALUES] = [0; NUMBER_OF_COLOR_VALUES];
    create_lookup_table(&mut lookup_table, &input_params);
    _basic_operations::apply_average_lookup_table(image, &lookup_table);
}

fn create_lookup_table(lookup_table: &mut [u8; NUMBER_OF_COLOR_VALUES], input_params: &ThresholdInput) {
    for i in 0..NUMBER_OF_COLOR_VALUES {
        lookup_table[i] = if (i as u8) < input_params.threshold { input_params.minimum } else { input_params.maximum };
    }
}