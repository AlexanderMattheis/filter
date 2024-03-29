use crate::system::defaults::messages::errors;

pub struct ChannelsInput {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
    pub luminance: bool,
}

impl ChannelsInput {
    pub fn new(channels: &RgbaChannelsInput) -> ChannelsInput {
        return ChannelsInput {
            red: channels.red,
            green: channels.green,
            blue: channels.blue,
            alpha: channels.alpha,
            luminance: false,
        };
    }
}

#[derive(Copy, Clone)]
pub struct RgbaChannelsInput {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
}

impl RgbaChannelsInput {
    pub fn new(channels: &ChannelsInput) -> RgbaChannelsInput {
        if channels.luminance {
            errors::print_warning(errors::NOT_USED_LUMINANCE);
        }

        return RgbaChannelsInput {
            red: channels.red,
            green: channels.green,
            blue: channels.blue,
            alpha: channels.alpha,
        };
    }
}