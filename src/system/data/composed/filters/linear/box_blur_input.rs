use crate::system::data::composed::filters::linear::_linear_input::LinearInput;

pub struct BoxBlurInput {
    pub linear_input: LinearInput,
    pub iterations: u16,
}