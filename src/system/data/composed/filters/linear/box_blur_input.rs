use crate::system::data::elementary::channels_input::RgbaChannelsInput;
use crate::system::defaults::types::border_handling::BorderHandling;

pub struct BoxBlurInput {
    pub background_color: [u8; 4],
    pub border_handling: BorderHandling,
    pub channels: RgbaChannelsInput,
    pub iterations: u16,
    pub radius_horizontal: usize,
    pub radius_vertical: usize
}