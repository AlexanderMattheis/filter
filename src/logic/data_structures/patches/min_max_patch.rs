use std::collections::VecDeque;
use crate::logic::algorithm_params::NUM_OF_VALUES_SUM;

pub struct MinMaxPatch1D {
    rgba: VecDeque<([u8; 4], u16)>,

    most_extreme: ([u8; 4], u16),
    second_extreme: ([u8; 4], u16),

    most_extreme_index: isize,
    second_extreme_index: isize,

    compute_minima: bool,

    size: usize,
}

impl MinMaxPatch1D {
    pub fn new(radius: usize, compute_minima: bool) -> MinMaxPatch1D {
        let length = 2 * radius + 1;
        let default_value = if compute_minima { NUM_OF_VALUES_SUM as u16 } else { 0 };

        return MinMaxPatch1D {
            rgba: VecDeque::with_capacity(length),

            most_extreme: ([0; 4], default_value),
            second_extreme: ([0; 4], default_value),

            most_extreme_index: 0,
            second_extreme_index: 0,

            compute_minima,

            size: length,
        };
    }

    pub fn insert(&mut self, value: &([u8; 4], u16)) {
        if self.most_extreme_index < 0 {
            self.most_extreme = self.second_extreme;
            self.most_extreme_index = self.second_extreme_index;

            self.second_extreme_index = get_second_highest_index(&self.rgba);
            self.second_extreme = self.rgba[self.second_extreme_index as usize];
        } else if self.second_extreme_index < 0 {
            self.second_extreme_index = get_second_highest_index(&self.rgba);
            self.second_extreme = self.rgba[self.second_extreme_index as usize];
        }

        if self.compute_minima && self.most_extreme.1 >= value.1 {
            self.set_values(value);
        } else if !self.compute_minima && self.most_extreme.1 <= value.1 {
            self.set_values(value);
        }

        if self.rgba.len() >= self.size {
            self.rgba.pop_front();

            self.second_extreme_index -= 1;
            self.most_extreme_index -= 1;
        }

        self.rgba.push_back(*value);
    }

    fn set_values(&mut self, value: &([u8; 4], u16)) {
        self.second_extreme = self.most_extreme;
        self.most_extreme = *value;

        self.second_extreme_index = self.most_extreme_index;
        self.most_extreme_index = self.rgba.len() as isize;
    }

    pub fn clear(&mut self) {
        self.rgba.clear();
        self.most_extreme = ([0; 4], 0);
        self.second_extreme = ([0; 4], 0);

        self.most_extreme_index = 0;
        self.second_extreme_index = 0;
    }

    pub fn get_extrema(&mut self) -> ([u8; 4], u16) {
        return self.most_extreme;
    }
}

fn get_second_highest_index(queue: &VecDeque<([u8; 4], u16)>) -> isize {
    let mut highest = ([0; 4], 0);

    let mut highest_index = 0;
    let mut second_highest_index = 0;

    for i in 0..queue.len() {
        if highest >= queue[i] {
            highest = queue[i];

            second_highest_index = highest_index;
            highest_index = i;
        }
    }

    return second_highest_index as isize;
}

