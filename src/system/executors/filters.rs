use image::{DynamicImage, GenericImageView};

use crate::logic::actions::filters::linear::box_blur;
use crate::system::defaults::messages::errors;
use crate::system::io::input::filters::linear::box_blur_parser;

pub fn compute_box_blur(image: &DynamicImage, params: &String, output_file_name_path: &String) {
    let mut temp_image_1 = DynamicImage::new_rgba8(image.width(), image.height());
    let mut temp_image_2 = DynamicImage::new_rgba8(image.width(), image.height());
    let input_params = box_blur_parser::parse_params(params, &image.dimensions());

    box_blur::run(image, &mut temp_image_1, &mut temp_image_2, &input_params);
    match temp_image_2.save(output_file_name_path) {
        Err(error) => errors::print_error_and_quit(errors::FAILED_SAVING_IMAGE, Some(error.to_string().as_str())),
        _ => {}
    }
}
