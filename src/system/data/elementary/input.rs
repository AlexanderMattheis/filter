use crate::system::data::elementary::channels_input::ChannelsInput;

pub struct Input {
    pub channels: Option<ChannelsInput>,
    pub logarithmic: Option<bool>,
    pub cumulative: Option<bool>
}
