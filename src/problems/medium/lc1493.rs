#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut zero_count, mut max_count) = (0, 0, 0);
        for right in 0..nums.len() {
            if nums[right] == 0 {
                zero_count += 1;
            }
            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }
            max_count = max_count.max(right - left);
        }
        max_count as i32
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_exapmle_1() {
        let input_vec = vec![1, 1, 0, 1];
        let output_val = 3;
        let res = Solution::longest_subarray(input_vec);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_exapmle_2() {
        let input_vec = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let output_val = 5;
        let res = Solution::longest_subarray(input_vec);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_exapmle_3() {
        let input_vec = vec![1, 1, 1];
        let output_val = 2;
        let res = Solution::longest_subarray(input_vec);
        assert_eq!(output_val, res);
    }
}
