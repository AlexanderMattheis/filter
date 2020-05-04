pub struct PixelBound {
    pub lower: u8,
    pub higher: u8,
}

pub struct RgbaPixelBounds {
    pub red_bound: PixelBound,
    pub green_bound: PixelBound,
    pub blue_bound: PixelBound,
    pub alpha_bound: PixelBound,
}

impl RgbaPixelBounds {
    pub fn new(lower: u8, higher: u8) -> RgbaPixelBounds {
        return RgbaPixelBounds {
            red_bound: PixelBound { lower, higher },
            green_bound: PixelBound { lower, higher },
            blue_bound: PixelBound { lower, higher },
            alpha_bound: PixelBound { lower, higher },
        };
    }
}