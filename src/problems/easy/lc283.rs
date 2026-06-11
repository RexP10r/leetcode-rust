#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut input_vec = vec![0, 1, 0, 3, 12];
        let changed_vec = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut input_vec);
        assert_eq!(input_vec, changed_vec);
    }
    #[test]
    fn test_example_2() {
        let mut input_vec = vec![0];
        let changed_vec = vec![0];
        Solution::move_zeroes(&mut input_vec);
        assert_eq!(input_vec, changed_vec);
    }
}
