use crate::system::data::elementary::channels_input::RgbaChannelsInput;

pub struct HistogramEqualizationInput {
    pub channels: RgbaChannelsInput,
    pub accumulate_roots: bool,
    pub per_channel: bool
}