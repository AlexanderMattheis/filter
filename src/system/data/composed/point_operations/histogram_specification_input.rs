use crate::system::data::elementary::channels_input::RgbaChannelsInput;

pub struct HistogramSpecificationInput {
    pub channels: RgbaChannelsInput,
    pub per_channel: bool
}