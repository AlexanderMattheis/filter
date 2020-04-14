use crate::system::data::elementary::channels_input::ChannelsInput;

pub struct Input {
    pub channels: Option<ChannelsInput>,
    pub cumulative: Option<bool>,
    // tells if should be divided by value
    pub division: Option<bool>,
    pub logarithmic: Option<bool>,
    pub maximum: Option<u8>,
    pub minimum: Option<u8>,
    pub quantile_low: Option<f64>,
    pub quantile_high: Option<f64>,
    pub threshold: Option<u8>,
    pub value: Option<f64>,
}

impl Input {
    pub fn new() -> Input {
        return Input {
            channels: None,
            cumulative: None,
            division: None,
            logarithmic: None,
            maximum: None,
            minimum: None,
            quantile_low: None,
            quantile_high: None,
            threshold: None,
            value: None,

        };
    }
}
