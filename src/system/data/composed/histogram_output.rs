use crate::system::defaults::algorithm_params::NUMBER_OF_HISTOGRAM_BINS;

pub struct HistogramOutput {
    pub red_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub green_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub blue_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub alpha_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub luminance_data: [f64; NUMBER_OF_HISTOGRAM_BINS]
}

impl HistogramOutput {
    pub fn new() -> HistogramOutput {
        return HistogramOutput {
            red_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
            green_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
            blue_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
            alpha_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
            luminance_data: [0.0; NUMBER_OF_HISTOGRAM_BINS],
        };
    }
}