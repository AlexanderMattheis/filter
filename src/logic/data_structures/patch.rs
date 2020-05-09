use std::collections::LinkedList;

pub struct Patch1D {
    red: LinkedList<u8>,
    green: LinkedList<u8>,
    blue: LinkedList<u8>,
    alpha: LinkedList<u8>,
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
            size: length,
            average_factor: 1.0 / (length as f64),
        };
    }

    pub fn insert_red_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.red, value, self.size);
    }

    pub fn insert_green_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.green, value, self.size);
    }

    pub fn insert_blue_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.blue, value, self.size);
    }

    pub fn insert_alpha_at_back(&mut self, value: u8) {
        insert_value_at_back(&mut self.alpha, value, self.size);
    }

    pub fn average_red(&self) -> u8 {
        return average(&self.red, self.average_factor);
    }

    pub fn average_green(&self) -> u8 {
        return average(&self.green, self.average_factor);
    }

    pub fn average_blue(&self) -> u8 {
        return average(&self.blue, self.average_factor);
    }

    pub fn average_alpha(&self) -> u8 {
        return average(&self.alpha, self.average_factor);
    }

    pub fn clear(&mut self) {
        self.red.clear();
        self.green.clear();
        self.blue.clear();
        self.alpha.clear();
    }
}

fn insert_value_at_back(list: &mut LinkedList<u8>, value: u8, maximum_len: usize) {
    if list.len() >= maximum_len {
        list.pop_front();
    }

    list.push_back(value);
}

fn average(values: &LinkedList<u8>, average_factor: f64) -> u8 {
    let mut sum: u32 = 0;

    for value in values {
        sum = sum + (*value as u32);
    }

    return ((sum as f64) * average_factor).round() as u8;
}
