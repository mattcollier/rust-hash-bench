extern crate hex;
extern crate sha2;
use sha2::{Sha256, Digest};
use node_bindgen::derive::node_bindgen;

#[node_bindgen]
pub fn hasher(seed: String, count: i32) -> Vec<String> {
    let mut results = Vec::with_capacity(count as usize);
    let mut s = seed;
    for _ in 0..count {
        let mut hasher = Sha256::new();
        hasher.update(s);
        let result = hasher.finalize();
        s = hex::encode(result);
        results.push(s.clone());
    }

    results
}