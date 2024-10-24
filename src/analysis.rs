use std::cmp::Ord;
use std::time::Duration;

pub fn median<T: Ord + Copy>(vec: &[T]) -> T {
    let mut sorted = vec.to_vec();
    sorted.sort();
    sorted[sorted.len() / 2]
}

pub fn mean(vec: &[Duration]) -> Duration {
    let accum = vec.iter().map(|e| *e).reduce(|acc, e| acc + e);
    accum.unwrap() / (vec.len() as u32)
}
