use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
