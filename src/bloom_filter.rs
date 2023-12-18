use sha2::{Sha256, Digest};
use shabal::{Shabal256, Shabal192};
// use bit_set::BitSet;
use bitvec::prelude::*;

pub struct BloomFilter {
    bitset: BitVec<u8, Msb0>,
    size: usize
}

impl BloomFilter {
    fn _get_hash_slot_1(&self, val: &str) -> u128 {
        let mut hasher = Sha256::new();
        hasher.update(val);
        let hash = hasher.finalize();

        let mut hash_int: u128 = 0;

        // hashing lib returns array of bytes, convert into number
        for i in 0..16 {
            hash_int = (hash_int << 8) + hash[i] as u128;
        }

        let bucket = hash_int % self.size as u128;

        return bucket;
    }

    fn _get_hash_slot_2(&self, val: &str) -> u128 {
        let mut hasher2 = Shabal192::new();
        hasher2.update(val);
        let hash2 = hasher2.finalize();
        
        let mut hash_int_2: u128 = 0;

        for i in 0..16 {
            hash_int_2 = (hash_int_2 << 8) + hash2[i] as u128;
        }

        let bucket_2 = hash_int_2 % self.size as u128;
        return bucket_2;
    }

    fn get_buckets(&self, val: &str) -> Vec<usize> {
        let mut buckets = Vec::new();

        let bucket_1 = self._get_hash_slot_1(val);
        let bucket_2 = self._get_hash_slot_2(val);

        buckets.push(bucket_1 as usize);
        buckets.push(bucket_2 as usize);

        return buckets;

    }

    pub fn new(size: usize) -> Self {
        BloomFilter {
            bitset: bitvec![u8, Msb0; 0; size],
            size: size
        }
    }

    pub fn set(&mut self, val: &str) {
        let buckets = self.get_buckets(val);

        for bucket in buckets {
            self.bitset.set(bucket, true)
        }
    }

    pub fn get(&self, val: &str) -> bool {
        let buckets = self.get_buckets(val);

        // println!("{:?}", buckets);

        let mut exists = true;

        for bucket in buckets {
            exists &= self.bitset.get(bucket).as_deref().unwrap();
        }

        return exists;
    }
}