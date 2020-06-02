use std::collections::VecDeque;

use crate::logic::data_structures::kernel::Kernel1D;

pub struct AveragePatch1D {
    red: VecDeque<u8>,
    green: VecDeque<u8>,
    blue: VecDeque<u8>,
    alpha: VecDeque<u8>,

    sum_red: u32,
    sum_green: u32,
    sum_blue: u32,
    sum_alpha: u32,

    kernel: Kernel1D,

    size: usize,
    average_factor: f64,
}

impl AveragePatch1D {
    pub fn new(radius: usize, kernel: Option<Kernel1D>) -> AveragePatch1D {
        let length = 2 * radius + 1;

        return AveragePatch1D {
            red: VecDeque::with_capacity(length),
            green: VecDeque::with_capacity(length),
            blue: VecDeque::with_capacity(length),
            alpha: VecDeque::with_capacity(length),

            sum_red: 0,
            sum_green: 0,
            sum_blue: 0,
            sum_alpha: 0,

            kernel: kernel.unwrap_or(Kernel1D::default()),

            size: length,
            average_factor: 1.0 / (length as f64),
        };
    }

    pub fn insert_red_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.red, &mut self.sum_red, value, self.size);
    }

    pub fn insert_green_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.green, &mut self.sum_green, value, self.size);
    }

    pub fn insert_blue_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.blue, &mut self.sum_blue, value, self.size);
    }

    pub fn insert_alpha_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.alpha, &mut self.sum_alpha, value, self.size);
    }

    pub fn average_red(&self) -> u8 {
        return average(self.sum_red, self.average_factor);
    }

    pub fn average_green(&self) -> u8 {
        return average(self.sum_green, self.average_factor);
    }

    pub fn average_blue(&self) -> u8 {
        return average(self.sum_blue, self.average_factor);
    }

    pub fn average_alpha(&self) -> u8 {
        return average(self.sum_alpha, self.average_factor);
    }

    pub fn weighted_average_red(&self) -> u8 {
        return weighted_average(&self.red, &self.kernel);
    }

    pub fn weighted_average_green(&self) -> u8 {
        return weighted_average(&self.green, &self.kernel);
    }

    pub fn weighted_average_blue(&self) -> u8 {
        return weighted_average(&self.blue, &self.kernel);
    }

    pub fn weighted_average_alpha(&self) -> u8 {
        return weighted_average(&self.alpha, &self.kernel);
    }

    pub fn clear(&mut self) {
        self.red.clear();
        self.green.clear();
        self.blue.clear();
        self.alpha.clear();

        self.sum_red = 0;
        self.sum_green = 0;
        self.sum_blue = 0;
        self.sum_alpha = 0;
    }
}

fn insert_value_at_back(queue: &mut VecDeque<u8>, sum: &mut u32, value: u8, maximum_len: usize) {
    if queue.len() >= maximum_len {
        *sum = *sum - (queue[0] as u32);
        queue.pop_front();
    }

    queue.push_back(value);
    *sum = *sum + (value as u32);  // could be enhanced by using a lookup table with respect to the size of the patch
}

fn average(sum: u32, average_factor: f64) -> u8 {
    return ((sum as f64) * average_factor).round() as u8;
}

fn weighted_average(values: &VecDeque<u8>, kernel: &Kernel1D) -> u8 {
    let mut weighted_sum = 0.0;

    for i in 0..values.len() {
        weighted_sum += (values[i] as f64) * kernel.weights[i];
    }

    return ((weighted_sum as f64) * kernel.divisor_factor).round() as u8;
}
