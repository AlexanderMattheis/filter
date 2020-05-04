use crate::logic::algorithm_params::NUMBER_OF_INPUT_CHANNELS;

#[derive(Copy, Clone)]
pub struct StatisticsOutput {
    pub min: u8,
    pub max: u8,
    pub average: f64,
    pub variance: f64,
    pub median: u8,
    pub contrast: u8,

    // since 257 different values are possible (the set can be empty or contain values between 0-255)
    pub dynamics: u16
}

impl StatisticsOutput {
    pub fn new() -> [StatisticsOutput; NUMBER_OF_INPUT_CHANNELS] {
        return [StatisticsOutput {
            min: 255,
            max: 0,
            average: 0.0,
            variance: 0.0,
            median: 0,
            contrast: 0,
            dynamics: 0
        }; NUMBER_OF_INPUT_CHANNELS];
    }
}