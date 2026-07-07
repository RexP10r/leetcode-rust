#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max_count = 1;
        let mut current_count = 1;
        for i in 1..s.len() {
            let temp = current_count + 1;
            current_count = if s.as_bytes()[i] == s.as_bytes()[i - 1] {
                temp
            } else {
                1
            };
            max_count = max_count.max(current_count)
        }
        max_count
    }
}
#[cfg(test)]
mod test {
    use crate::problems::easy::lc1446::Solution;

    #[test]
    fn test_example_1() {
        let input = "leetcode".to_string();
        let expected = 2;
        let res = Solution::max_power(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input = "abbcccddddeeeeedcba".to_string();
        let expected = 5;
        let res = Solution::max_power(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_3() {
        let input = " ".to_string();
        let expected = 1;
        let res = Solution::max_power(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_4() {
        let input = "cc".to_string();
        let expected = 2;
        let res = Solution::max_power(input);
        assert_eq!(expected, res);
    }
}
