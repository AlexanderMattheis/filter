use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::logic::actions::filters::border_handling::BorderHandling;
use crate::logic::actions::filters::non_linear::_non_linear_filter;
use crate::logic::actions::filters::non_linear::_non_linear_filter::LookupTable;
use crate::logic::algorithm_params::{NUM_OF_VALUES, NUM_OF_VALUES_SUM};
use crate::logic::data_structures::column_histogram_array::ColumnHistogramArray;
use crate::logic::data_structures::patches::median_grayscale_patch::MedianHistogramPatch;
use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::non_linear::median_filter_input::MedianFilterInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;

/// Perreault, Simon, and Patrick Hébert.
/// "Median filtering in constant time."
/// IEEE transactions on image processing 16.9 (2007): 2389-2394.
/// DOI: https://www.doi.org/10.1109/TIP.2007.902329
pub fn run(image: &DynamicImage, empty_image: &mut DynamicImage, input_params: &MedianFilterInput) {
    let mut lookup_table: LookupTable = [[0; NUM_OF_VALUES]; NUM_OF_VALUES_SUM];
    let mut column_histograms: ColumnHistogramArray = ColumnHistogramArray::new(image.dimensions().0 as usize,
                                                                                input_params.filter_input.radius_horizontal);

    _non_linear_filter::create_lookup_table(&mut lookup_table);
    compute_median(image, empty_image, &lookup_table, &mut column_histograms, &input_params.filter_input);
}

fn compute_median(image: &DynamicImage, empty_image: &mut DynamicImage, lookup_table: &LookupTable,
                  column_histograms: &mut ColumnHistogramArray, filter_input: &FilterInput) {
    let dim = image.dimensions();
    let dimensions = (dim.0 as i32, dim.1 as i32);

    let border_handling = BorderHandling::new(&filter_input.border_handling);
    let mut median_patch = MedianHistogramPatch::new(filter_input.radius_horizontal, filter_input.radius_vertical);

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            fill(image, &border_handling, &(u, v), &dimensions,
                 &mut median_patch, lookup_table, column_histograms, &filter_input);

            let median = median_patch.median_grayscale();
            empty_image.put_pixel(u as u32, v as u32, Rgba { 0: [median, median, median, 255] });
        }
        median_patch.clear();  // in a new line there are new values
    }
}

fn fill(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32), dimensions: &(i32, i32),
        median_patch: &mut MedianHistogramPatch, lookup_table: &LookupTable, column_histograms: &mut ColumnHistogramArray,
        filter_input: &FilterInput) {
    let horizontal = filter_input.radius_horizontal as i32;
    let vertical = filter_input.radius_horizontal as i32;

    if position.0 == 0 && position.1 == 0 {  // first position initialization
        init_first_position(image, &border_handling, position, &dimensions, median_patch, lookup_table, column_histograms, filter_input);
    } else if position.0 > 0 && position.1 == 0 {  // first line initialization
        init_first_line(image, &border_handling, &(position.0 + horizontal, position.1), &dimensions, median_patch, lookup_table, column_histograms, filter_input);
    } else if position.0 == 0 && position.1 > 0 {  // new line initialization
        init_subsequent_start_position(image, &border_handling, &(position.0, position.1 + vertical), &dimensions, median_patch, lookup_table, column_histograms, filter_input);
    } else {
        set_patch(image, &border_handling, &(position.0 + horizontal, position.1 + vertical), &dimensions, median_patch, lookup_table, column_histograms, filter_input);
    }
}

fn init_first_position(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                       dimensions: &(i32, i32), median_patch: &mut MedianHistogramPatch, lookup_table: &LookupTable,
                       column_histograms: &mut ColumnHistogramArray, filter_params: &FilterInput) {
    let vertical = filter_params.radius_vertical as i32;
    let horizontal = filter_params.radius_horizontal as i32;

    for i in -vertical..=vertical {
        for j in -horizontal..=horizontal {
            let pixel_value = get_pixel_value(image, border_handling, i, j, position, dimensions, filter_params);
            let rgba_average_value = get_average(&pixel_value, &filter_params.channels, lookup_table);

            column_histograms.add(j, rgba_average_value);
            median_patch.add(rgba_average_value);
        }
    }
}

fn get_pixel_value(image: &DynamicImage, border_handling: &BorderHandling, i: i32, j: i32,
                   position: &(i32, i32), dimensions: &(i32, i32), filter_params: &FilterInput) -> [u8; 4] {
    let pixel_value;

    if position.0 + j < 0 || position.0 + j >= dimensions.0 || position.1 + i < 0 || position.1 + i >= dimensions.1 {
        pixel_value = (border_handling.get_pixel)(image, position.0 + j, position.1 + i, &filter_params.background_color);
    } else {
        pixel_value = image.get_pixel((position.0 + j) as u32, (position.1 + i) as u32).0;
    }

    return pixel_value;
}

fn get_average(pixel_value: &[u8; 4], channels: &RgbaChannelsInput, lookup_table: &LookupTable) -> u8 {
    let mut sum = 0;
    let mut count = 0;

    if channels.red {
        sum = lookup_table[sum as usize][pixel_value[0] as usize];
        count += 1;
    }

    if channels.green {
        sum = lookup_table[sum as usize][pixel_value[1] as usize];
        count += 1;
    }

    if channels.blue {
        sum = lookup_table[sum as usize][pixel_value[2] as usize];
        count += 1;
    }

    if channels.alpha {
        sum = lookup_table[sum as usize][pixel_value[3] as usize];
        count += 1;
    }

    return ((sum as f64) / (count as f64)).round() as u8;
}

pub fn init_first_line(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                       dimensions: &(i32, i32), median_patch: &mut MedianHistogramPatch, lookup_table: &LookupTable,
                       column_histograms: &mut ColumnHistogramArray, filter_params: &FilterInput) {
    let vertical = filter_params.radius_horizontal as i32;
    let patch_width = median_patch.width() as i32;

    for i in -vertical..=vertical {
        // remove
        let old_pixel_value = get_pixel_value(image, border_handling, i, -patch_width, position, dimensions, filter_params);
        let old_rgba_average_value = get_average(&old_pixel_value, &filter_params.channels, lookup_table);

        // add
        let new_pixel_value = get_pixel_value(image, border_handling, i, 0, position, dimensions, filter_params);
        let new_rgba_average_value = get_average(&new_pixel_value, &filter_params.channels, lookup_table);

        column_histograms.add(position.0, new_rgba_average_value);
        median_patch.replace(old_rgba_average_value, new_rgba_average_value);
    }
}

pub fn init_subsequent_start_position(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                                      dimensions: &(i32, i32), median_patch: &mut MedianHistogramPatch, lookup_table: &LookupTable,
                                      column_histograms: &mut ColumnHistogramArray, filter_params: &FilterInput) {
    let horizontal = filter_params.radius_horizontal as i32;
    let patch_height = median_patch.height() as i32;

    for j in -horizontal..=horizontal {
        let prev_pixel_value = get_pixel_value(image, border_handling, -patch_height, j, position, dimensions, filter_params);
        let prev_rgba_average_value = get_average(&prev_pixel_value, &filter_params.channels, lookup_table);  // could be optimized with second lookup-table

        let next_pixel_value = get_pixel_value(image, border_handling, 0, j, position, dimensions, filter_params);
        let next_rgba_average_value = get_average(&next_pixel_value, &filter_params.channels, lookup_table);

        column_histograms.replace(position.0 + j, prev_rgba_average_value, next_rgba_average_value);  // absolute
        median_patch.add_column(column_histograms.get(j));  // ok, since after each line there is a clear
    }
}

pub fn set_patch(image: &DynamicImage, border_handling: &BorderHandling, position: &(i32, i32),
                 dimensions: &(i32, i32), median_patch: &mut MedianHistogramPatch, lookup_table: &LookupTable,
                 column_histograms: &mut ColumnHistogramArray, filter_params: &FilterInput) {
    let patch_height = median_patch.height() as i32;
    let patch_width = median_patch.width() as i32;

    let prev_pixel_value = get_pixel_value(image, border_handling, -patch_height, 0, position, dimensions, filter_params);
    let prev_rgba_average_value = get_average(&prev_pixel_value, &filter_params.channels, lookup_table);  // could be optimized with second lookup-table

    let next_pixel_value = get_pixel_value(image, border_handling, 0, 0, position, dimensions, filter_params);
    let next_rgba_average_value = get_average(&next_pixel_value, &filter_params.channels, lookup_table);

    column_histograms.replace(position.0, prev_rgba_average_value, next_rgba_average_value);  // absolute
    median_patch.replace_column(column_histograms.get(position.0 - patch_width), column_histograms.get(position.0));
}
