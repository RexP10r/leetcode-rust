#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left: usize = 1;
        for right in 1..nums.len() {
            if nums[left-1] != nums[right] {
                nums[left] = nums[right];
                left += 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![1, 1, 2];
        let res_nums = vec![1, 2];
        let res_val = 2;
        let answer_val = Solution::remove_duplicates(&mut nums);
        assert_eq!(answer_val, res_val);
        assert_eq!(nums[0..(res_val) as usize], res_nums);
    }
    #[test]
    fn test_example_2() {
        let mut nums = vec![0, 0, 1, 1, 2, 2, 3, 3, 4];
        let res_nums = vec![0, 1, 2, 3, 4];
        let res_val = 5;
        let answer_val = Solution::remove_duplicates(&mut nums);
        assert_eq!(answer_val, res_val);
        assert_eq!(nums[0..(res_val) as usize], res_nums);
    }
}
