use crate::system::data::elementary::channels_input::{ChannelsInput, RgbaChannelsInput};

pub struct HistogramDefaults;
pub struct StatisticsDefaults;

// point-operations
pub struct BrightnessDefauls;
pub struct ContrastDefaults;
pub struct InversionDefaults;
pub struct ThresholdDefaults;

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

// point-operations
impl BrightnessDefauls {
    pub const CHANNELS_INPUT: RgbaChannelsInput = RgbaChannelsInput {
        red: true,
        green: true,
        blue: true,
        alpha: false
    };

    pub const VALUE: i16 = 0;
}

impl ContrastDefaults {
    pub const CHANNELS_INPUT: RgbaChannelsInput = RgbaChannelsInput {
        red: true,
        green: true,
        blue: true,
        alpha: false
    };

    pub const VALUE: f64 = 1.0;
}

impl InversionDefaults {
    pub const CHANNELS_INPUT: RgbaChannelsInput = RgbaChannelsInput {
        red: false,
        green: false,
        blue: false,
        alpha: false
    };
}

impl ThresholdDefaults {
    pub const THRESHOLD: u8 = 127;
    pub const MINIMUM: u8 = 0;
    pub const MAXIMUM: u8 = 255;
}