use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::logic::actions::filters::border_handling::BorderHandling;
use crate::logic::algorithm_params::{NUM_OF_VALUES, NUM_OF_VALUES_SUM};
use crate::logic::data_structures::patches::maximum_patch::MaximumPatch1D;
use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::non_linear::maximum_filter_input::MaximumFilterInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;

type LookupTable = [[u16; NUM_OF_VALUES]; NUM_OF_VALUES_SUM];

/*
Hint: Could possibly be optimized vertically by over 30% with the horizontal optimization idea.
*/
pub fn run(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &MaximumFilterInput) {
    let mut lookup_table: LookupTable = [[0; NUM_OF_VALUES]; NUM_OF_VALUES_SUM];

    create_lookup_table(&mut lookup_table);
    compute_maxima(image, empty_image, &lookup_table, &input_params.filter_input);
}

fn create_lookup_table(lookup_table: &mut LookupTable) {
    for i in 0..NUM_OF_VALUES_SUM {
        for j in 0..NUM_OF_VALUES {
            lookup_table[i][j] = (i + j) as u16;
        }
    }
}

fn compute_maxima(image: &DynamicImage, empty_image: &mut DynamicImage, lookup_table: &LookupTable, filter_input: &FilterInput) {
    let dim = image.dimensions();
    let dimensions = (dim.0 as i32, dim.1 as i32);

    let border_handling = BorderHandling::new(&filter_input.border_handling);
    let mut maximum_patch = MaximumPatch1D::new(filter_input.radius_horizontal);

    for v in 0..dimensions.1 as i32 {
        for u in 0..dimensions.0 as i32 {
            fill_patch(image, &border_handling, &(u, v), &dimensions,
                       &mut maximum_patch, &lookup_table, &filter_input);

            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: maximum_patch.get_max().0 });
        }
        maximum_patch.clear();  // in a new line there are new values
    }
}

fn fill_patch(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32), dimensions: &(i32, i32),
              maximum_patch: &mut MaximumPatch1D, lookup_table: &LookupTable, filter_input: &FilterInput) {
    let radius_horizontal = filter_input.radius_horizontal as i32;

    if position.0 == 0 {
        init_maximum_patch(image, &border_handling, &position, &dimensions, maximum_patch, lookup_table, filter_input);
    } else {
        maximum_patch.insert(&get_vertical_maximum(image, border_handling, radius_horizontal, position, dimensions, lookup_table, filter_input));
    }
}

fn init_maximum_patch(image: &DynamicImage, border_handling: &BorderHandling,
                      position: &(i32, i32), dimensions: &(i32, i32), maximum_patch: &mut MaximumPatch1D,
                      lookup_table: &LookupTable, filter_params: &FilterInput) {
    let horizontal = filter_params.radius_horizontal as i32;

    for j in -horizontal..=horizontal {
        maximum_patch.insert(&get_vertical_maximum(image, border_handling, j, position, dimensions, lookup_table, filter_params));
    }
}

fn get_vertical_maximum(image: &DynamicImage, border_handling: &BorderHandling, index: i32,
                        position: &(i32, i32), dimensions: &(i32, i32),
                        lookup_table: &LookupTable, filter_params: &FilterInput) -> ([u8; 4], u16) {
    let radius_vertical = filter_params.radius_vertical as i32;

    let mut maximum = ([0; 4], 0);

    for i in -radius_vertical..=radius_vertical {
        let pixel_value;

        if position.0 + index < 0 || position.0 + index >= dimensions.0 || position.1 + i < 0 || position.1 + i >= dimensions.1 {
            pixel_value = (border_handling.get_pixel)(image, position.0 + index, position.1 + i, &filter_params.background_color);
        } else {
            pixel_value = image.get_pixel((position.0 + index) as u32, (position.1 + i) as u32).0;
        }

        let pixel_value_sum = get_sum(&pixel_value, &filter_params.channels, lookup_table);

        if pixel_value_sum > maximum.1 {
            maximum = (pixel_value, pixel_value_sum);
        }
    }

    return maximum;
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

    return sum as u16;
}
