use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::logic::actions::filters::border_handling::BorderHandling;
use crate::logic::actions::filters::non_linear::_non_linear_filter;
use crate::logic::actions::filters::non_linear::_non_linear_filter::LookupTable;
use crate::logic::algorithm_params::{NUM_OF_VALUES, NUM_OF_VALUES_SUM};
use crate::logic::data_structures::patches::min_max_patch::MinMaxPatch1D;
use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::non_linear::min_max_filter_input::MinMaxFilterInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;

/// Hint: Same problem as with the Median Filter with multiple dimensions.
/// As an approximation for multidimensional ranking a one dimensional ranking parameter (="sum") is used,
/// since practically there was no big visual difference.
pub fn run(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &MinMaxFilterInput, compute_minima: bool) {
    let mut lookup_table: LookupTable = [[0; NUM_OF_VALUES]; NUM_OF_VALUES_SUM];

    _non_linear_filter::create_lookup_table(&mut lookup_table);
    compute_extrema(image, empty_image, &lookup_table, &input_params.filter_input, compute_minima);
}

fn compute_extrema(image: &DynamicImage, empty_image: &mut DynamicImage, lookup_table: &LookupTable, filter_input: &FilterInput, compute_minima: bool) {
    let dim = image.dimensions();
    let dimensions = (dim.0 as i32, dim.1 as i32);

    let border_handling = BorderHandling::new(&filter_input.border_handling);
    let mut min_max_patch = MinMaxPatch1D::new(filter_input.radius_horizontal, compute_minima);

    for v in 0..dimensions.1 as i32 {
        for u in 0..dimensions.0 as i32 {
            fill_patch(image, &border_handling, &(u, v), &dimensions,
                       &mut min_max_patch, &lookup_table, &filter_input, compute_minima);

            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: min_max_patch.get_extrema().0 });
        }
        min_max_patch.clear();  // in a new line there are new values
    }
}

fn fill_patch(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32), dimensions: &(i32, i32),
              min_max_patch: &mut MinMaxPatch1D, lookup_table: &LookupTable, filter_input: &FilterInput, compute_minima: bool) {
    let radius_horizontal = filter_input.radius_horizontal as i32;

    if position.0 == 0 {
        init_min_max_patch(image, &border_handling, &position, &dimensions, min_max_patch, lookup_table, filter_input, compute_minima);
    } else {
        min_max_patch.insert(&get_vertical_extrema(
            image, border_handling, radius_horizontal, position, dimensions, lookup_table, filter_input, compute_minima));
    }
}

fn init_min_max_patch(image: &DynamicImage, border_handling: &BorderHandling,
                      position: &(i32, i32), dimensions: &(i32, i32), min_max_patch: &mut MinMaxPatch1D,
                      lookup_table: &LookupTable, filter_params: &FilterInput, compute_minima: bool) {
    let horizontal = filter_params.radius_horizontal as i32;

    for j in -horizontal..=horizontal {
        min_max_patch.insert(&get_vertical_extrema(image, border_handling, j, position, dimensions, lookup_table, filter_params, compute_minima));
    }
}

fn get_vertical_extrema(image: &DynamicImage, border_handling: &BorderHandling, index: i32,
                        position: &(i32, i32), dimensions: &(i32, i32),
                        lookup_table: &LookupTable, filter_params: &FilterInput,
                        compute_minima: bool) -> ([u8; 4], u16) {
    let radius_vertical = filter_params.radius_vertical as i32;

    let mut extrema = ([0; 4], if compute_minima { NUM_OF_VALUES_SUM as u16 } else { 0 });

    for i in -radius_vertical..=radius_vertical {
        let pixel_value = _non_linear_filter::get_pixel_value(image, border_handling, i, index, position, dimensions, filter_params);
        let pixel_value_sum = get_sum(&pixel_value, &filter_params.channels, lookup_table);

        if compute_minima && pixel_value_sum < extrema.1 {
            extrema = (pixel_value, pixel_value_sum);
        } else if !compute_minima && pixel_value_sum > extrema.1 {
            extrema = (pixel_value, pixel_value_sum);
        }
    }

    return extrema;
}

fn get_sum(pixel_value: &[u8; 4], channels: &RgbaChannelsInput, lookup_table: &LookupTable) -> u16 {
    let mut sum = 0;

    if channels.red {
        sum = lookup_table[sum as usize][pixel_value[0] as usize];
    }

    if channels.green {
        sum = lookup_table[sum as usize][pixel_value[1] as usize];
    }

    if channels.blue {
        sum = lookup_table[sum as usize][pixel_value[2] as usize];
    }

    if channels.alpha {
        sum = lookup_table[sum as usize][pixel_value[3] as usize];
    }

    return sum;
}