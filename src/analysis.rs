use std::time::Duration;

pub fn median(vec: &[Duration]) -> Duration {
    let mut sorted = vec.to_vec();
    sorted.sort();
    sorted[sorted.len() / 2]
}

pub fn mean(vec: &[Duration]) -> Duration {
    let accum = vec.iter().fold(Duration::from_millis(0), |acc, e| acc + *e);
    accum / (vec.len() as u32)
}
