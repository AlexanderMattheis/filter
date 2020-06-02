use crate::logic::algorithm_params::NUM_OF_VALUES;

pub struct MedianHistogramPatch {
    histogram: [u32; NUM_OF_VALUES],
    median_pos: u32,
    width: usize,
    height: usize,
}

impl MedianHistogramPatch {
    pub fn new(radius_horizontal: usize, radius_vertical: usize) -> MedianHistogramPatch {
        let height = 2 * radius_vertical + 1;
        let width = 2 * radius_horizontal + 1;
        let max_size = height * width;

        return MedianHistogramPatch {
            histogram: [0; 256],
            median_pos: (max_size / 2) as u32,
            width,
            height,
        };
    }

    pub fn width(&mut self) -> usize {
        return self.width;
    }

    pub fn height(&mut self) -> usize {
        return self.height;
    }

    pub fn add(&mut self, value: u8) {
        self.histogram[value as usize] += 1;
    }

    pub fn replace(&mut self, old_value: u8, new_value: u8) {
        self.histogram[old_value as usize] -= 1;
        self.histogram[new_value as usize] += 1;
    }

    pub fn add_column(&mut self, values: [u32; NUM_OF_VALUES]) {
        for i in 0..values.len() {
            self.histogram[i] += values[i];
        }
    }

    pub fn replace_column(&mut self, old_values: [u32; NUM_OF_VALUES], new_values: [u32; NUM_OF_VALUES]) {
        for i in 0..new_values.len() {
            self.histogram[i] -= old_values[i];
            self.histogram[i] += new_values[i];
        }
    }

    pub fn median_grayscale(&self) -> u8 {
        let mut accumulated_histogram = self.histogram;

        if accumulated_histogram[0] >= self.median_pos {
            return 0;
        }

        for i in 1..accumulated_histogram.len() {  // check auto-vectorization
            accumulated_histogram[i] += accumulated_histogram[i - 1];  // integration

            if accumulated_histogram[i] >= self.median_pos {
                return i as u8;
            }
        }

        return 255;
    }

    pub fn clear(&mut self) {
        self.histogram = [0; NUM_OF_VALUES];  // check auto-vectorization
    }
}
