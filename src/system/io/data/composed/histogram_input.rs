use crate::system::io::data::elementary::channels_input::ChannelsInput;

pub struct HistogramInput {
    pub channels: ChannelsInput,
    pub logarithmic: bool
}