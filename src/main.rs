use std::result;

use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng, Rng};
use std::mem;

pub mod bloom_filter;

fn generate_random_string(length: usize) -> String {
    // todo - add seed
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), length);
    return string
}

fn main() {
    // Create a bitset. It's technically a byte array, but in which one operates with individuals bit individually
    // for now hardcoded to 8
    let mut bloom_filter = bloom_filter::BloomFilter::new();

    println!("{}", bloom_filter.get("bla_bla"));
    // so if it says false, then it means 100% it's not in the set. true means it might be there, but not guaranteed.
    println!("{}", bloom_filter.get("bla_bla_6"));

    println!("{}", generate_random_string(64));

    // set candidates
    let mut candidates: Vec<String> = Vec::new();

    const CANDIDATES: usize = 50_000;

    for _ in 0..CANDIDATES {
        candidates.push(generate_random_string(64));
    }

    // println!("mem: {:?}", mem::size_of_val(&candidates));

    // half insert
    for i in 0..CANDIDATES/2 {
        let mut false_positives = 0;

        for i in CANDIDATES/2..CANDIDATES {
            let existance = bloom_filter.get(candidates.get(i).unwrap());

            if existance {
                false_positives += 1;
            }
        }

        let fp_rate = false_positives as f32 / ((CANDIDATES/2) as f32) * 100.0;
        println!("Size: {i}, FP rate: {fp_rate}%");
        bloom_filter.set(candidates.get(i).unwrap());
    }
}