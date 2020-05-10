use std::collections::LinkedList;

pub struct Patch1D {
    red: LinkedList<u8>,
    green: LinkedList<u8>,
    blue: LinkedList<u8>,
    alpha: LinkedList<u8>,

    sum_red: u32,
    sum_green: u32,
    sum_blue: u32,
    sum_alpha: u32,

    size: usize,
    average_factor: f64,
}

impl Patch1D {
    pub fn new(length: usize) -> Patch1D {
        return Patch1D {
            red: LinkedList::new(),
            green: LinkedList::new(),
            blue: LinkedList::new(),
            alpha: LinkedList::new(),

            sum_red: 0,
            sum_green: 0,
            sum_blue: 0,
            sum_alpha: 0,

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

fn insert_value_at_back(list: &mut LinkedList<u8>, sum: &mut u32, value: u8, maximum_len: usize) {
    if list.len() >= maximum_len {
        *sum = *sum - (list.pop_front().unwrap() as u32);
    }

    list.push_back(value);
    *sum = *sum + (value as u32);
}

fn average(sum: u32, average_factor: f64) -> u8 {
    return ((sum as f64) * average_factor).round() as u8;
}
