use crate::system::data::elementary::channels_input::ChannelsInput;

pub struct HistogramDefaults;

impl HistogramDefaults {
    pub const CHANNELS_INPUT: ChannelsInput = ChannelsInput {
        red: false,
        green: false,
        blue: false,
        alpha: false,
        luminance: true
    };
    pub const LOGARITHMIC: bool = false;
    pub const CUMULATIVE: bool = false;
}

