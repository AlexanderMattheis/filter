use crate::system::defaults::algorithm_params::NUMBER_OF_HISTOGRAM_BINS;

pub struct HistogramOutput {
    pub red_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub green_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub blue_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub alpha_data: [f64; NUMBER_OF_HISTOGRAM_BINS],
    pub luminance_data: [f64; NUMBER_OF_HISTOGRAM_BINS]
}