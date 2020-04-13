use crate::system::data::elementary::channels_input::{ChannelsInput, RgbaChannelsInput};

pub struct HistogramDefaults;
pub struct StatisticsDefaults;
pub struct BrightnessDefauls;

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

impl StatisticsDefaults {
    pub const CHANNELS_INPUT: ChannelsInput = ChannelsInput {
        red: false,
        green: false,
        blue: false,
        alpha: false,
        luminance: true
    };
}

impl BrightnessDefauls {
    pub const CHANNELS_INPUT: RgbaChannelsInput = RgbaChannelsInput {
        red: true,
        green: true,
        blue: true,
        alpha: false
    };

    pub const VALUE: i16 = 0;
    pub const NEGATIVE: bool = false;
}