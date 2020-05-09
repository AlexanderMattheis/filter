use crate::system::data::elementary::channels_input::ChannelsInput;
use crate::system::defaults::types::border_handling_type::BorderHandlingType;

pub struct Input {
    pub background_color: Option<[u8; 4]>,
    pub border_handling: Option<BorderHandlingType>,
    pub channels: Option<ChannelsInput>,
    pub cumulative: Option<bool>,
    // tells if should be divided by value
    pub division: Option<bool>,
    pub enhanced: Option<bool>,
    pub iterations: Option<u16>,
    pub logarithmic: Option<bool>,
    pub maximum: Option<u8>,
    pub minimum: Option<u8>,
    pub per_channel: Option<bool>,
    pub quantile_low: Option<f64>,
    pub quantile_high: Option<f64>,
    pub radius_horizontal: Option<usize>,
    pub radius_vertical: Option<usize>,
    pub threshold: Option<u8>,
    pub value: Option<f64>,
}

impl Input {
    pub fn new() -> Input {
        return Input {
            background_color: None,
            border_handling: None,
            channels: None,
            cumulative: None,
            division: None,
            enhanced: None,
            iterations: None,
            logarithmic: None,
            maximum: None,
            minimum: None,
            per_channel: None,
            quantile_low: None,
            quantile_high: None,
            radius_horizontal: None,
            radius_vertical: None,
            threshold: None,
            value: None,

        };
    }
}
