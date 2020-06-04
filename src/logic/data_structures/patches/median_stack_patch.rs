use std::collections::{VecDeque, BTreeMap};
use std::f64::MAX;

pub struct MedianStackPatch {
    rgba: VecDeque<[u8; 4]>,
    maximum_size: usize,
}

impl MedianStackPatch {
    pub fn new(radius_horizontal: usize, radius_vertical: usize) -> MedianStackPatch {
        let width = 2 * radius_horizontal + 1;
        let height = 2 * radius_vertical + 1;
        let maximum_size = width * height;

        return MedianStackPatch {
            rgba: VecDeque::with_capacity(maximum_size),
            maximum_size,
        };
    }

    pub fn insert_rgba_at_back(&mut self, value: &[u8; 4]) {
        if self.rgba.len() > self.maximum_size {
            self.rgba.pop_front();
        }

        self.rgba.push_back(*value);
    }

    pub fn median_rgba(&mut self) -> [u8; 4] {
        let mut minimizing_vector = [0; 4];
        let mut minimum_distance_distance = MAX;

        for i in 0..self.rgba.len() {
            let measurement_vector = self.rgba[i];
            let mut pairwise_distance = 0.0;

            for j in 0..self.rgba.len() {
                let vector = self.rgba[j];
                pairwise_distance += self.get_distance(&measurement_vector, &vector);
            }

            if pairwise_distance < minimum_distance_distance {
                minimum_distance_distance = pairwise_distance;
                minimizing_vector = measurement_vector;
            }
        }

        return minimizing_vector;
    }

    fn get_distance(&mut self, vector_a: &[u8; 4], vector_b: &[u8; 4]) -> f64 {
        let distance = get_euclidean_distance(&vector_a, &vector_b);
        return distance;
    }

    pub fn clear(&mut self) {
        self.rgba.clear();
    }
}

fn get_euclidean_distance(vec_a: &[u8; 4], vec_b: &[u8; 4]) -> f64 {
    let mut squared_distance = 0;

    for i in 0..4 {
        squared_distance += (vec_a[i] as i32 - vec_b[i] as i32).pow(2);
    }

    return (squared_distance as f64).sqrt();
}
