#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut left = 0;
        let mut right = 0;
        let mut res_vec: Vec<String> = Vec::new();
        while left < nums.len() {
            right = left;
            let start = nums[left];
            while right + 1 < nums.len() && nums[right + 1] == nums[right] + 1 {
                right += 1;
            }
            if nums[right] == start {
                res_vec.push(start.to_string());
            } else {
                res_vec.push(format!("{}->{}", start, nums[right]));
            }
            left = right + 1;
        }
        res_vec
    }
}
