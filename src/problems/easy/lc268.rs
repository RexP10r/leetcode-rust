#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; nums.len() + 1];
        nums.into_iter().for_each(|num| seen[num as usize] = true);
        seen.iter()
            .position(|&is_seen| !is_seen)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}
#[cfg(test)]
mod test {
    use crate::problems::easy::lc268::Solution;

    #[test]
    fn test_example_1() {
        let input = vec![3, 0, 1];
        let expected = 2;
        let res = Solution::missing_number(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input = vec![0, 1];
        let expected = 2;
        let res = Solution::missing_number(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_3() {
        let input = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let expected = 8;
        let res = Solution::missing_number(input);
        assert_eq!(expected, res);
    }
}
