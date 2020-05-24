use crate::system::data::composed::filters::filter_input::FilterInput;
use crate::system::data::composed::filters::non_linear::min_max_filter_input::MinMaxFilterInput;
use crate::system::data::elementary::input::Input;
use crate::system::io::input::_basic_parser;
use crate::system::io::input::filters::filter_parser;

pub fn parse_params(params: &String, image_dimensions: &(u32, u32)) -> MinMaxFilterInput {
    let input: Input = _basic_parser::parse_params(params);
    let filter_input: FilterInput = filter_parser::parse_params(&input, image_dimensions);

    return MinMaxFilterInput {
        filter_input
    };
}
