pub struct Kernel1D {
    pub weights: Vec<f64>,
    pub divisor_factor: f64,
}

impl Kernel1D {
    pub fn new(weights: Vec<f64>, divisor: f64) -> Kernel1D {
        return Kernel1D {
            weights,
            divisor_factor: 1.0 / divisor
        };
    }

    pub fn default() -> Kernel1D {
        return Kernel1D {
            weights: Vec::new(),
            divisor_factor: 1.0
        };
    }
}