#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left: usize = 0;

        for right in 0..nums.len() {
            if nums[right] != val {
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
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let res_nums = vec![2, 2];
        let res_val = 2;
        let answer_val = Solution::remove_element(&mut nums, val);
        assert_eq!(answer_val, res_val);
        assert_eq!(nums[0..(res_val) as usize], res_nums);
    }
    #[test]
    fn test_example_2() {
        let mut nums = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let res_nums = vec![0,1,3,0,4];
        let res_val = 5;
        let answer_val = Solution::remove_element(&mut nums, val);
        assert_eq!(answer_val, res_val);
        assert_eq!(nums[0..(res_val) as usize], res_nums);
    }
}
