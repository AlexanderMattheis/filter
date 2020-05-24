pub struct BorderHandlingTypes;

pub struct ChannelTypes;

pub const HEXADECIMAL_CHARACTERS: &'static str = "0123456789ABCDEFabcdef";

impl BorderHandlingTypes {
    pub const PADDING_CONSTANT_VALUE: &'static str = "padding-constant-value";
    pub const PADDING_EXTEND: &'static str = "padding-extend";
    pub const PADDING_MIRROR: &'static str = "padding-mirror";
    pub const PADDING_PERIODICALLY: &'static str = "padding-periodically";
}

impl ChannelTypes {
    pub const RED: char = 'r';
    pub const GREEN: char = 'g';
    pub const BLUE: char = 'b';
    pub const ALPHA: char = 'a';
    pub const LUMINANCE: char = 'l';
}
