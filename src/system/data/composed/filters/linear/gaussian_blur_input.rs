use crate::system::data::composed::filters::linear::_linear_input::LinearInput;
use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::defaults::types::border_handling_type::BorderHandlingType;

pub struct GaussianBlurInput {
    pub linear_input: LinearInput
}