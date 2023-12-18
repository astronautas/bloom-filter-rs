use std::result;

use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng, Rng};
use std::mem;
use bitvec::prelude::*;

pub mod bloom_filter;

fn generate_random_string(length: usize) -> String {
    // todo - add seed
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), length);
    return string
}

fn main() {
    // set candidates
    let mut candidates: Vec<String> = Vec::new();

    const CANDIDATES: usize = 10_000;

    for _ in 0..CANDIDATES {
        candidates.push(generate_random_string(24));
    }

    // // set half, test other half
    let mut bloom_filter = bloom_filter::BloomFilter::new(1_000_000);

    for i in 0..CANDIDATES/2 {
        bloom_filter.set(candidates.get(i).unwrap());
    }

    let mut false_positives = 0;
    for i in CANDIDATES/2..CANDIDATES {
        let existance = bloom_filter.get(candidates.get(i).unwrap());

        if existance {
            false_positives += 1;
        }
    }

    let fp_rate = false_positives as f32 / ((CANDIDATES/2) as f32) * 100.0;
    println!("Size: {CANDIDATES} / 2, FP rate: {fp_rate}%");
}