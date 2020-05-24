use image::DynamicImage;

use crate::logic::algorithm_params::NUM_OF_VALUES;
use crate::logic::data_structures::lookup_table::RgbaLookupTable;
use crate::system::data::composed::point_operations::threshold_input::ThresholdInput;

pub fn run(image: &mut DynamicImage, input_params: &ThresholdInput) {
    let mut lookup_table: [u8; NUM_OF_VALUES] = [0; NUM_OF_VALUES];
    create_lookup_table(&mut lookup_table, &input_params);
    RgbaLookupTable::apply_average_lookup_table(image, &lookup_table);
}

fn create_lookup_table(lookup_table: &mut [u8; NUM_OF_VALUES], input_params: &ThresholdInput) {
    for i in 0..NUM_OF_VALUES {
        lookup_table[i] = if (i as u8) < input_params.threshold { input_params.minimum } else { input_params.maximum };
    }
}