use sha2::{Sha256, Digest};
use shabal::{Shabal256, Shabal192};

pub struct BloomFilter {
    bitset: u8,
}

impl BloomFilter {
    fn get_buckets(val: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(val);
        let hash = hasher.finalize();

        let mut hash_int: u128 = 0;

        for i in 0..16 {
            hash_int = (hash_int << 8) + hash[i] as u128;
        }

        let bucket = hash_int % 8;

        let mut hasher2 = Shabal192::new();
        hasher2.update(val);
        let hash2 = hasher2.finalize();
        
        let mut hash_int_2: u128 = 0;

        for i in 0..16 {
            hash_int_2 = (hash_int_2 << 8) + hash2[i] as u128;
        }

        let bucket_2 = hash_int_2 % 8;

        let mut buckets = Vec::new();
        buckets.push(bucket as u8);
        buckets.push(bucket_2 as u8);

        return buckets;

    }

    pub fn new() -> Self {
        BloomFilter {
            bitset: 0
        }
    }

    pub fn set(&mut self, val: &str) {
        let buckets = BloomFilter::get_buckets(val);

        for bucket in buckets {
            // getting the right bit
            // duplication
            let mut mask: u8 = 1;

            for i in 0..bucket {
                mask = mask << 1;
            }

            self.bitset = self.bitset | mask;
        }
    }

    pub fn get(&self, val: &str) -> bool {
        let buckets = BloomFilter::get_buckets(val);

        // println!("{:?}", buckets);

        let mut exists = true;

        for bucket in buckets {
            // getting the right bit
            let mut mask: u8 = 1;

            for i in 0..bucket {
                mask = mask << 1;
            }

            // if even a single bit is not set, it means for sure the value hasn't been added and in the
            // process hashed by bloom filter
            exists &= ((mask & self.bitset) == mask)
        }

        return exists;
    }
    // fn remove()
}