use std::collections::HashMap;

pub struct Kernel1D {
    pub kernel: HashMap<i32, i32>,
    pub divisor_factor: f64,
}

impl Kernel1D {
    pub fn new(kernel: HashMap<i32, i32>, divisor: i32) -> Kernel1D {
        return Kernel1D {
            kernel,
            divisor_factor: 1.0 / divisor as f64
        };
    }

    pub fn default() -> Kernel1D {
        return Kernel1D {
            kernel: HashMap::new(),
            divisor_factor: 1.0
        };
    }
}