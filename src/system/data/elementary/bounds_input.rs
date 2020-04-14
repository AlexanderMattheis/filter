pub struct PixelBound {
    pub lower: u8,
    pub higher: u8,
}

pub struct RgbaPixelBoundsInput {
    pub red_bound: PixelBound,
    pub green_bound: PixelBound,
    pub blue_bound: PixelBound,
    pub alpha_bound: PixelBound,
}

impl RgbaPixelBoundsInput {
    pub fn new(lower: u8, higher: u8) -> RgbaPixelBoundsInput {
        return RgbaPixelBoundsInput {
            red_bound: PixelBound { lower, higher },
            green_bound: PixelBound { lower, higher },
            blue_bound: PixelBound { lower, higher },
            alpha_bound: PixelBound { lower, higher },
        };
    }
}