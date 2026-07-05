#![allow(dead_code)]
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s_bytes = s.into_bytes();
        let mut left: usize = 0;
        let mut map: HashMap<u8, usize> = HashMap::new();
        let mut res = 0;
        for right in 0..s_bytes.len() {
            if let Some(old_idx) = map.insert(s_bytes[right], right) {
                if old_idx >= left {
                    left = old_idx + 1;
                }
            }
            res = res.max(right - left + 1);
        }
        res as i32
    }
}
#[cfg(test)]
mod test {
    use crate::problems::medium::lc3::Solution;

    #[test]
    fn test_example_1() {
        let input = "abcabcbb".to_string();
        let expected = 3;
        let res = Solution::length_of_longest_substring(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input = "au".to_string();
        let expected = 2;
        let res = Solution::length_of_longest_substring(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_3() {
        let input = "dvdf".to_string();
        let expected = 3;
        let res = Solution::length_of_longest_substring(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_4() {
        let input = " ".to_string();
        let expected = 1;
        let res = Solution::length_of_longest_substring(input);
        assert_eq!(expected, res);
    }
}
