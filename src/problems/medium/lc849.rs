#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let (mut left, mut right) = (usize::MAX, 0);
        let mut res = 0;
        while right < seats.len() {
            if seats[right] == 1 {
                res = if left == usize::MAX {
                    right
                } else {
                    res.max((right - left) / 2)
                };
                left = right;
            }
            right += 1;
        }
        res.max(seats.len() - left - 1) as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::time::{Duration, Instant};
    struct TestConf {
        input: Vec<i32>,
        expected: i32,
        num_iterations: u32,
    }
    impl TestConf {
        fn new(input: Vec<i32>, expected: i32, num_iterations: u32) -> Self {
            Self {
                input,
                expected,
                num_iterations,
            }
        }
    }

    fn test_cycle(config: TestConf) {
        let mut total_duration = Duration::new(0, 0);

        for _ in 0..config.num_iterations {
            let current_input = config.input.clone();

            let now = Instant::now();
            let res = Solution::max_dist_to_closest(current_input);
            total_duration += now.elapsed();

            assert_eq!(config.expected, res);
        }

        let average_duration = total_duration / config.num_iterations;

        println!("==========================================");
        println!("Total duration: {:?}", total_duration);
        println!("Expected Speed: {:?}", average_duration);
        println!("==========================================");
    }
    #[test]
    fn test_example_1() {
        let input = vec![1, 0, 0, 0, 1, 0, 1];
        let expected = 2;
        let num_iterations = 10000000;
        test_cycle(TestConf::new(input, expected, num_iterations));
    }
    #[test]
    fn test_example_2() {
        let input = vec![1, 0, 0, 0];
        let expected = 3;
        let num_iterations = 10000000;
        test_cycle(TestConf::new(input, expected, num_iterations));
    }
    #[test]
    fn test_example_3() {
        let input = vec![0, 0, 1];
        let expected = 2;
        let num_iterations = 10000000;
        test_cycle(TestConf::new(input, expected, num_iterations));
    }
}
