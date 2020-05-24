use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::linear::box_blur_input::BoxBlurInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::{BoxBlurDefaults};
use crate::system::io::input::_basic_parser;
use crate::system::io::input::filters::filter_parser;

pub fn parse_params(params: &String, image_dimensions: &(u32, u32)) -> BoxBlurInput {
    let input: Input = _basic_parser::parse_params(params);
    let linear_input: FilterInput = filter_parser::parse_params(&input, image_dimensions);

    let iterations = match input.iterations {
        Some(iterations) => iterations,
        _ => BoxBlurDefaults::ITERATIONS
    };

    return BoxBlurInput {
        filter_input: linear_input,
        iterations,
    };
}
