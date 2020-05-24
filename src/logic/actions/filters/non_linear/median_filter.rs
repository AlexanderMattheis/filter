use image::DynamicImage;
use crate::system::data::composed::filters::non_linear::min_max_filter_input::MinMaxFilterInput;
use crate::logic::actions::filters::non_linear::min_max_filter::LookupTable;
use crate::logic::algorithm_params::{NUM_OF_VALUES, NUM_OF_VALUES_SUM};
use crate::system::data::composed::filters::non_linear::median_filter_input::MedianFilterInput;

pub fn run(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &MedianFilterInput) {
    let mut lookup_table: LookupTable = [[0; NUM_OF_VALUES]; NUM_OF_VALUES_SUM];

    create_lookup_table(&mut lookup_table);
    compute_extrema(image, empty_image, &lookup_table, &input_params.filter_input, compute_minima);
}