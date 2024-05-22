use  super::hash;

extern crate alloc;
use alloc::vec::Vec;
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    hasher: hash::DefaultHasher,
    capacity: usize,
}
impl<K: AsRef<[u8]>+ core::cmp::PartialEq, V> HashMap<K, V> {
    pub fn new() -> Self {
        let capacity = 16; 
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(Vec::new());
        }
        HashMap {
            buckets,
            hasher: hash::DefaultHasher::new(),
            capacity,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hasher.hash(&key.as_ref()) as usize % self.capacity;
        let bucket = &mut self.buckets[index];
        for &mut (ref k, ref mut v) in bucket.iter_mut() {
            if k == &key {
                *v = value;
                return;
            }
        }
        bucket.push((key, value));
    }


    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets.iter().flat_map(|bucket| bucket.iter())
            .map(|(k, v)| (k, v))
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hasher.hash(key.as_ref()) as usize % self.capacity;
        let bucket = &self.buckets[index];
        for &(ref k, ref v) in bucket.iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }
}
