use crate::system::data::composed::filters::linear::_linear_input::LinearInput;
use crate::system::data::composed::filters::linear::box_blur_input::BoxBlurInput;
use crate::system::data::composed::filters::linear::gaussian_blur_input::GaussianBlurInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::data::elementary::input::Input;
use crate::system::defaults::cli::actions_params_defaults::BoxBlurDefaults;
use crate::system::defaults::messages::errors;
use crate::system::io::input::_basic_parser;
use crate::system::io::input::filters::linear::_linear_input_parser;

pub fn parse_params(params: &String, image_dimensions: &(u32, u32)) -> GaussianBlurInput {
    let input: Input = _basic_parser::parse_params(params);
    let linear_input: LinearInput = _linear_input_parser::parse_params(&input, image_dimensions);

    return GaussianBlurInput {
        linear_input
    };
}
