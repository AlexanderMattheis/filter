pub struct ChannelsInput {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
    pub luminance: bool,
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