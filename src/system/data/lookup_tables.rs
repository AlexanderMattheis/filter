use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;

pub struct LookupTables {
    pub red_bound: [u8; NUMBER_OF_COLOR_VALUES],
    pub green_bound: [u8; NUMBER_OF_COLOR_VALUES],
    pub blue_bound: [u8; NUMBER_OF_COLOR_VALUES],
    pub alpha_bound: [u8; NUMBER_OF_COLOR_VALUES],
}

impl LookupTables {
    pub fn new() -> LookupTables {
        return LookupTables {
            red_bound: [0; 256],
            green_bound: [0; 256],
            blue_bound: [0; 256],
            alpha_bound: [0; 256]
        }
    }
}