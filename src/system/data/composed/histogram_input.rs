use crate::system::data::elementary::channels_input::ChannelsInput;

pub struct HistogramInput {
    pub channels: ChannelsInput,
    pub logarithmic: bool,
    pub cumulative: bool
}