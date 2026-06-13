#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_sum = Vec::with_capacity(nums.len());
        prefix_sum.push(nums[0]);
        for i in 1..nums.len() {
            prefix_sum.push(nums[i] + prefix_sum[i - 1]);
        }
        let mut hash_map = HashMap::<i32, i32>::new();
        let mut answer = 0;
        for i in 0..prefix_sum.len() {
            if prefix_sum[i] == k {
                answer += 1;
            }
            if let Some(count) = hash_map.get(&(prefix_sum[i]-k)) {
                answer += count;
            }
            if let Some(count) = hash_map.get_mut(&prefix_sum[i]) {
                *count += 1;
            } else {
                hash_map.insert(prefix_sum[i], 1);
            }
        }
        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_vec = vec![1, 1, 1];
        let input_val = 2;
        let output_val = 2;
        let res = Solution::subarray_sum(input_vec, input_val);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_example_2() {
        let input_vec = vec![1, 2, 3];
        let input_val = 3;
        let output_val = 2;
        let res = Solution::subarray_sum(input_vec, input_val);
        assert_eq!(output_val, res);
    }
        #[test]
    fn test_example_3() {
        let input_vec = vec![3,4,-4,3,1];
        let input_val = 4;
        let output_val = 3;
        let res = Solution::subarray_sum(input_vec, input_val);
        assert_eq!(output_val, res);
    }

}
