#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let nums: Vec<u8> = nums.into_iter().map(|n| n as u8).collect();
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() {
            k -= 1 - unsafe { *nums.get_unchecked(right) as i32 };
            if k < 0 {
                k += 1 - unsafe { *nums.get_unchecked(left) as i32};
                left += 1;
            }
            right += 1;
        }
        (right - left) as i32
    }
}
#[cfg(test)]
mod test {
    use crate::problems::medium::lc1004::Solution;

    #[test]
    fn test_example_1() {
        let input_vec = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let input_val = 2;
        let expected = 6;
        let res = Solution::longest_ones(input_vec, input_val);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input_vec = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let input_val = 3;
        let expected = 10;
        let res = Solution::longest_ones(input_vec, input_val);
        assert_eq!(expected, res);
    }
}
