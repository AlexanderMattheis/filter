use crate::system::data::composed::filters::filter_input::FilterInput;

pub struct BoxBlurInput {
    pub filter_input: FilterInput,
    pub iterations: u16,
}