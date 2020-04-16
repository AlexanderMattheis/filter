use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;

pub struct RgbaHistogram {
    pub red_data: [f64; NUMBER_OF_COLOR_VALUES],
    pub green_data: [f64; NUMBER_OF_COLOR_VALUES],
    pub blue_data: [f64; NUMBER_OF_COLOR_VALUES],
    pub alpha_data: [f64; NUMBER_OF_COLOR_VALUES],
}

impl RgbaHistogram {
    pub fn new() -> RgbaHistogram {
        return RgbaHistogram {
            red_data: [0.0; NUMBER_OF_COLOR_VALUES],
            green_data: [0.0; NUMBER_OF_COLOR_VALUES],
            blue_data: [0.0; NUMBER_OF_COLOR_VALUES],
            alpha_data: [0.0; NUMBER_OF_COLOR_VALUES]
        };
    }
}