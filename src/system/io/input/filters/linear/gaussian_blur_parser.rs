use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::linear::gaussian_blur_input::GaussianBlurInput;
use crate::system::data::elementary::input::Input;
use crate::system::io::input::_basic_parser;
use crate::system::io::input::filters::filter_parser;

pub fn parse_params(params: &String, image_dimensions: &(u32, u32)) -> GaussianBlurInput {
    let input: Input = _basic_parser::parse_params(params);
    let filter_input: FilterInput = filter_parser::parse_params(&input, image_dimensions);

    return GaussianBlurInput {
        filter_input
    };
}
