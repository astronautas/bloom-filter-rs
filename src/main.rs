use sha2::{Sha256, Digest};

struct BloomFilter {
    bitset: u8,
    // size: u32
}

impl BloomFilter {
    fn get_bucket(val: &str) -> u8 {
        let mut hasher = Sha256::new();
        hasher.update(val);
        let hash = hasher.finalize();

        let mut hash_int: u128 = 0;

        for i in 0..16 {
            hash_int = (hash_int << 8) + hash[i] as u128;
        }

        let bucket = hash_int % 8;

        return bucket as u8;
    }

    fn new() -> Self {
        BloomFilter {
            bitset: 0
        }
    }

    fn set(&mut self, val: &str) {
        let bucket = BloomFilter::get_bucket(val);

        // getting the right bit
        // duplication
        let mut mask: u8 = 1;

        for i in 0..bucket {
            mask = mask << 1;
        }

        self.bitset = self.bitset | mask;
    }

    fn get(&self, val: &str) -> bool {

        let bucket = BloomFilter::get_bucket(val);

        // getting the right bit
        let mut mask: u8 = 1;

        for i in 0..bucket {
            mask = mask << 1;
        }

        // e.g. only if all same bits are set in the mask (checking), then it means value
        // has been set
        return (mask & self.bitset) == mask
    }
    // fn remove()
}

fn main() {
    // Create a bitset. It's technically a byte array, but in which one operates with individuals bit individually
    // for now hardcoded to 8
    let mut bitset: u8 = 0;


    let mut bloom_filter = BloomFilter::new();

    bloom_filter.set("bla_bla");

    println!("{}", bloom_filter.get("bla_bla"));
    // so if it says false, then it means 100% it's not in the set. true means it might be there, but not guaranteed.
    println!("{}", bloom_filter.get("bla_bla_6"));
}
