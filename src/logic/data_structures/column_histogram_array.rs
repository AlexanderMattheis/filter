use crate::logic::algorithm_params::NUM_OF_VALUES;

pub struct ColumnHistogramArray {
    histograms: Vec<[u32; NUM_OF_VALUES]>,
    padding: usize,
}

impl ColumnHistogramArray {
    pub fn new(image_width: usize, radius_horizontal: usize) -> ColumnHistogramArray {
        return ColumnHistogramArray {
            histograms: vec![[0; NUM_OF_VALUES]; image_width as usize + 2 * radius_horizontal],
            padding: radius_horizontal,
        };
    }

    pub fn add(&mut self, index: i32, value: u8) {
        self.histograms[(index + self.padding as i32) as usize][value as usize] += 1;
    }

    pub fn replace(&mut self, index: i32, prev_value: u8, next_value: u8) {
        self.histograms[(index + self.padding as i32) as usize][prev_value as usize] -= 1;
        self.histograms[(index + self.padding as i32) as usize][next_value as usize] += 1;
    }

    pub fn get(&self, index: i32) -> [u32; NUM_OF_VALUES] {
        return self.histograms[(index + self.padding as i32) as usize];
    }
}