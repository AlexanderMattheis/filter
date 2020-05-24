use crate::logic::algorithm_params::NUM_OF_VALUES;

pub struct HistogramOutput {
    pub red_data: [f64; NUM_OF_VALUES],
    pub green_data: [f64; NUM_OF_VALUES],
    pub blue_data: [f64; NUM_OF_VALUES],
    pub alpha_data: [f64; NUM_OF_VALUES],
    pub luminance_data: [f64; NUM_OF_VALUES],
}

impl HistogramOutput {
    pub fn new() -> HistogramOutput {
        return HistogramOutput {
            red_data: [0.0; NUM_OF_VALUES],
            green_data: [0.0; NUM_OF_VALUES],
            blue_data: [0.0; NUM_OF_VALUES],
            alpha_data: [0.0; NUM_OF_VALUES],
            luminance_data: [0.0; NUM_OF_VALUES],
        };
    }
}