pub struct Kernel1D {
    pub weights: Vec<i32>,
    pub divisor_factor: f64,
}

impl Kernel1D {
    pub fn new(weights: Vec<i32>, divisor: i32) -> Kernel1D {
        return Kernel1D {
            weights,
            divisor_factor: 1.0 / divisor as f64
        };
    }

    pub fn default() -> Kernel1D {
        return Kernel1D {
            weights: Vec::new(),
            divisor_factor: 1.0
        };
    }
}