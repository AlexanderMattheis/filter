use crate::system::data::elementary::channels_input::ChannelsInput;

pub struct Input {
    pub channels: Option<ChannelsInput>,
    pub logarithmic: Option<bool>,
    pub cumulative: Option<bool>,
    pub value: Option<f64>,
    pub division: Option<bool>  // tells if should be divided by value
}
