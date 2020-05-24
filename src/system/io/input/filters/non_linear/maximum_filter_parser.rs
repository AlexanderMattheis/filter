use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::non_linear::maximum_filter_input::MaximumFilterInput;
use crate::system::data::elementary::input::Input;
use crate::system::io::input::_basic_parser;
use crate::system::io::input::filters::filter_parser;

pub fn parse_params(params: &String, image_dimensions: &(u32, u32)) -> MaximumFilterInput {
    let input: Input = _basic_parser::parse_params(params);
    let filter_input: FilterInput = filter_parser::parse_params(&input, image_dimensions);

    return MaximumFilterInput {
        filter_input
    };
}