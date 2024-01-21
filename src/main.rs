

use bitvec::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rand::rngs::StdRng;
use rand::SeedableRng;



pub mod bloom_filter;

fn generate_random_string(length: usize, rng: &mut StdRng) -> String {
    let string = Alphanumeric.sample_string(rng, length);
    return string;
}

fn main() {
    const CANDIDATES_COUNT: usize = 3;
    let mut candidates: Vec<String> = Vec::new();
    let mut generator = StdRng::from_seed([0; 32]);

    // We'll prepare CANDIDATES_COUNT random strings
    // and insert first CANDIDATES_COUNT / 2 of them into the bloom filter
    // we'll then check the rest of the strings for existance in the bloom filter
    // ideally, we should get 0 false positives
    for _ in 0..CANDIDATES_COUNT {
        candidates.push(generate_random_string(24, &mut generator));
    }

    let mut bloom_filter = bloom_filter::BloomFilter::new(8);

    for i in 0..CANDIDATES_COUNT / 2 {
        bloom_filter.insert(candidates.get(i).unwrap());
    }

    let mut false_positives = 0;
    for i in CANDIDATES_COUNT / 2..CANDIDATES_COUNT {
        let existance = bloom_filter.get(candidates.get(i).unwrap());

        if existance {
            false_positives += 1;
        }
    }

    let fp_rate: f32 = false_positives as f32 / ((CANDIDATES_COUNT / 2) as f32) * 100.0;
    println!("Size: {CANDIDATES_COUNT} / 2, FP rate: {fp_rate}%");
    
}
