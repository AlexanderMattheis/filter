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
            luminance: false
        };
    }
}

pub struct RgbaChannelsInput {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
}

impl RgbaChannelsInput {
    pub fn new(channels: &ChannelsInput) -> RgbaChannelsInput {
        return RgbaChannelsInput {
            red: channels.red,
            green: channels.green,
            blue: channels.blue,
            alpha: channels.alpha,
        };
    }
}