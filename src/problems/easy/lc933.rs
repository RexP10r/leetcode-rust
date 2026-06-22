#![allow(dead_code)]
use std::collections::VecDeque;
struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);

        while self.queue.front().map_or(false, |&front| front < t - 3000) {
            self.queue.pop_front();
        }

        self.queue.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut obj = RecentCounter::new();
        let input_vec = vec![1, 100, 3001, 3002];
        let output_vec = vec![1, 2, 3, 3];
        let res_vec: Vec<i32> = input_vec.into_iter().map(|t| obj.ping(t)).collect();
        assert_eq!(output_vec, res_vec);
    }
}
