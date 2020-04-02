use crate::system::io::data::elementary::channels_input::ChannelsInput;

pub struct Input {
    pub channels: Option<ChannelsInput>,
    pub logarithmic: Option<bool>
}
