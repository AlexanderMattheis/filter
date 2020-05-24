use std::collections::VecDeque;

pub struct MaximumPatch1D {
    rgba: VecDeque<([u8; 4], u16)>,

    highest: ([u8; 4], u16),
    second_highest: ([u8; 4], u16),

    highest_index: isize,
    second_highest_index: isize,

    size: usize,
}

impl MaximumPatch1D {
    pub fn new(radius: usize) -> MaximumPatch1D {
        let length = 2 * radius + 1;

        return MaximumPatch1D {
            rgba: VecDeque::new(),

            highest: ([0; 4], 0),
            second_highest: ([0; 4], 0),

            highest_index: 0,
            second_highest_index: 0,

            size: length,
        };
    }

    pub fn insert(&mut self, value: &([u8; 4], u16)) {
        if self.highest_index < 0 {
            self.highest = self.second_highest;
            self.highest_index = self.second_highest_index;

            self.second_highest_index = get_second_highest_index(&self.rgba);
            self.second_highest = self.rgba[self.second_highest_index as usize];
        } else if self.second_highest_index < 0 {
            self.second_highest_index = get_second_highest_index(&self.rgba);
            self.second_highest = self.rgba[self.second_highest_index as usize];
        }

        if self.highest.1 <= value.1 {
            self.second_highest = self.highest;
            self.highest = *value;

            self.second_highest_index = self.highest_index;
            self.highest_index = self.rgba.len() as isize;
        }

        if self.rgba.len() >= self.size {
            self.rgba.pop_front();

            self.second_highest_index -= 1;
            self.highest_index -= 1;
        }

        self.rgba.push_back(*value);
    }

    pub fn clear(&mut self) {
        self.rgba.clear();
        self.highest = ([0; 4], 0);
        self.second_highest = ([0; 4], 0);

        self.highest_index = 0;
        self.second_highest_index = 0;
    }

    pub fn get_max(&mut self) -> ([u8; 4], u16) {
        return self.highest;
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

