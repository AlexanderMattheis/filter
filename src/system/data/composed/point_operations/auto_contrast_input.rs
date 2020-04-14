use crate::system::data::elementary::channels_input::RgbaChannelsInput;

pub struct AutoContrastInput {
    pub channels: RgbaChannelsInput,
    pub quantile_low: f64,
    pub quantile_high: f64,
}
