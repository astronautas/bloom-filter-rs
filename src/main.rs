use sha2::{Sha256, Digest};

fn main() {
    // Create a bitset. It's technically a byte array, but in which one operates with individuals bit individually
    // for now hardcoded to 8
    let mut bitset: u8 = 0;

    let mut hasher = Sha256::new();
    let data = "!d5222";
    hasher.update(data);
    let hash = hasher.finalize();
    // println!("Binary hash: {:?}", hash);

    let mut hash_int: u128 = 0;

    for i in 0..16 {
        hash_int = (hash_int << 8) + hash[i] as u128;
    }
    
    println!("{hash_int}");
    
    let bucket = hash_int % 8;

    println!("{bucket}");

    // setting the rigth bit
    let mut mask: u8 = 1;

    for i in 0..bucket {
        mask = mask << 1;
    }

    bitset = bitset + mask;

    println!("{bitset}")
}
