use image::{DynamicImage, GenericImageView};

use crate::logic::data_structures::histogram::FloatingPointRgbaHistogram;
use crate::logic::data_structures::lookup_table::RgbaLookupTable;
use crate::system::data::composed::point_operations::histogram_specification_input::HistogramSpecificationInput;
use crate::logic::algorithm_params::NUM_OF_VALUES;

pub fn run(image: &mut DynamicImage, ref_image: &DynamicImage, input_params: &HistogramSpecificationInput) {
    let mut image_cdf = FloatingPointRgbaHistogram::new();
    let mut ref_image_cdf = FloatingPointRgbaHistogram::new();

    compute_cumulative_distributions(image, ref_image, &mut image_cdf, &mut ref_image_cdf);

    // apply table
    let mut lookup_tables: RgbaLookupTable = RgbaLookupTable::new();
    create_lookup_tables(input_params, &mut lookup_tables, &image_cdf, &ref_image_cdf);
    RgbaLookupTable::apply_lookup_tables(image, &lookup_tables, &input_params.channels);
}

fn compute_cumulative_distributions(image: &DynamicImage, ref_image: &DynamicImage,
                                    image_cdf: &mut FloatingPointRgbaHistogram, ref_image_cdf: &mut FloatingPointRgbaHistogram) {
    compute_histograms(image, ref_image, image_cdf, ref_image_cdf);

    let img_dimensions = (image.dimensions().0 * image.dimensions().1) as f64;
    let ref_img_dimensions = (ref_image.dimensions().0 * ref_image.dimensions().1) as f64;

    // compute cumulative histogram
    make_first_value_relative(image_cdf, img_dimensions);
    make_first_value_relative(ref_image_cdf, ref_img_dimensions);

    for i in 1..NUM_OF_VALUES {
        sum_up_relative_values(i, image_cdf, img_dimensions);
        sum_up_relative_values(i, ref_image_cdf, ref_img_dimensions);
    }
}

fn make_first_value_relative(image_cdf: &mut FloatingPointRgbaHistogram, img_dimensions: f64) {
    image_cdf.red_data[0] /= img_dimensions;
    image_cdf.green_data[0] /= img_dimensions;
    image_cdf.blue_data[0] /= img_dimensions;
    image_cdf.alpha_data[0] /= img_dimensions;
}

fn compute_histograms(image: &DynamicImage, ref_image: &DynamicImage,
                      image_cdf: &mut FloatingPointRgbaHistogram, ref_image_cdf: &mut FloatingPointRgbaHistogram) {
    let img_dimensions = image.dimensions();
    let ref_img_dimensions = ref_image.dimensions();

    let same_width = img_dimensions.0 == ref_img_dimensions.0;
    let same_height = img_dimensions.1 == ref_img_dimensions.1;

    if same_width && same_height {
        count_values_in_both(image, ref_image, image_cdf, ref_image_cdf);
    } else {
        count_values(image, image_cdf);
        count_values(ref_image, ref_image_cdf);
    }
}

fn count_values_in_both(image: &DynamicImage, ref_image: &DynamicImage,
                        image_cdf: &mut FloatingPointRgbaHistogram, ref_image_cdf: &mut FloatingPointRgbaHistogram) {
    let dimensions = image.dimensions();

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            count_values_at(u, v, image, image_cdf);
            count_values_at(u, v, ref_image, ref_image_cdf);
        }
    }
}

fn count_values(image: &DynamicImage, histogram: &mut FloatingPointRgbaHistogram) {
    let dimensions = image.dimensions();

    for v in 0..dimensions.1 {
        for u in 0..dimensions.0 {
            count_values_at(u, v, image, histogram);
        }
    }
}

fn count_values_at(u: u32, v: u32, image: &DynamicImage, histogram: &mut FloatingPointRgbaHistogram) {
    let pixel_value = image.get_pixel(u, v).0;

    let red_value = pixel_value[0] as usize;
    let green_value = pixel_value[1] as usize;
    let blue_value = pixel_value[2] as usize;
    let alpha_value = pixel_value[3] as usize;

    histogram.red_data[red_value] += 1.0;
    histogram.green_data[green_value] += 1.0;
    histogram.blue_data[blue_value] += 1.0;
    histogram.alpha_data[alpha_value] += 1.0;
}

fn sum_up_relative_values(index: usize, cdf: &mut FloatingPointRgbaHistogram, num_pixels: f64) {
    cdf.red_data[index] = (cdf.red_data[index] / num_pixels) + cdf.red_data[index - 1];
    cdf.green_data[index] = (cdf.green_data[index] / num_pixels) + cdf.green_data[index - 1];
    cdf.blue_data[index] = (cdf.blue_data[index] / num_pixels) + cdf.blue_data[index - 1];
    cdf.alpha_data[index] = (cdf.alpha_data[index] / num_pixels) + cdf.alpha_data[index - 1];
}

fn create_lookup_tables(input_params: &HistogramSpecificationInput, lookup_tables: &mut RgbaLookupTable,
                        image_cdf: &FloatingPointRgbaHistogram, ref_image_cdf: &FloatingPointRgbaHistogram) {
    if input_params.channels.red {
        create_lookup_table(&mut lookup_tables.red_bound, &image_cdf.red_data, &ref_image_cdf.red_data);
    }

    if input_params.channels.green {
        create_lookup_table(&mut lookup_tables.green_bound, &image_cdf.green_data, &ref_image_cdf.green_data);
    }

    if input_params.channels.blue {
        create_lookup_table(&mut lookup_tables.blue_bound, &image_cdf.blue_data, &ref_image_cdf.blue_data);
    }

    if input_params.channels.alpha {
        create_lookup_table(&mut lookup_tables.alpha_bound, &image_cdf.alpha_data, &ref_image_cdf.alpha_data);
    }
}

fn create_lookup_table(lookup_table: &mut [u8; NUM_OF_VALUES],
                       probability_density: &[f64; NUM_OF_VALUES], ref_probability_density: &[f64; NUM_OF_VALUES]) {
    for i in 0..NUM_OF_VALUES {
        for j in 0..NUM_OF_VALUES {
            if probability_density[i] <= ref_probability_density[j] {
                lookup_table[i] = j as u8;
                break;  // to get the first `j` (minimum)
            }
        }
    }
}