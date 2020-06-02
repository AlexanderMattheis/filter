use crate::logic::algorithm_params::{NUM_OF_VALUES, NUM_OF_VALUES_SUM};

pub type LookupTable = [[u16; NUM_OF_VALUES]; NUM_OF_VALUES_SUM];

pub fn create_lookup_table(lookup_table: &mut LookupTable) {
    for i in 0..NUM_OF_VALUES_SUM {
        for j in 0..NUM_OF_VALUES {
            lookup_table[i][j] = (i + j) as u16;
        }
    }
}