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

#[cfg(test)]
mod tests {
    use crate::analysis::mean;
    use crate::analysis::median;
    use crate::Duration;

    #[test]
    fn test_median_mean() {
        assert_eq!(median::<u32>(&[144, 21, 127]), 127);

        assert_eq!(
            Duration::new(97, 0),
            mean(
                &[144, 21, 126]
                    .iter()
                    .map(|x| Duration::new(*x, 0))
                    .collect::<Vec<_>>()
            )
        );
    }
}
