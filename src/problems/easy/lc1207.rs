#![allow(dead_code)]

struct Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let nums_counter = arr.iter().fold(HashMap::new(), |mut hm, num| {
            let _ = *hm.entry(num).and_modify(|c| *c += 1).or_insert(1);
            hm
        });
        let set: HashSet<_> = nums_counter.values().collect();
        set.len() == nums_counter.len()
    }
}

#[cfg(test)]
mod test {
    use crate::problems::easy::lc1207::Solution;

    #[test]
    fn test_example_1() {
        let input = vec![1, 2, 2, 1, 1, 3];
        let expected = true;
        let res = Solution::unique_occurrences(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input = vec![1, 2];
        let expected = false;
        let res = Solution::unique_occurrences(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_3() {
        let input = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let expected = true;
        let res = Solution::unique_occurrences(input);
        assert_eq!(expected, res);
    }
}
