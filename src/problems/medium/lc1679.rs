#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right, mut count) = (0, nums.len() - 1, 0);
        nums.sort();
        while left < right {
            let current_sum = nums[left] + nums[right];
            if current_sum == k {
                count += 1;
                left += 1;
                right -= 1;
            } else if current_sum < k {
                left += 1;
            } else {
                //current_sum > k
                right -= 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1(){
        let input_vec = vec![1,2,3,4];
        let input_val = 5;
        let output_val = 2;
        let res = Solution::max_operations(input_vec, input_val);
        assert_eq!(output_val, res);
    }
        #[test]
    fn test_example_2(){
        let input_vec = vec![3,1,3,4,3];
        let input_val = 6;
        let output_val = 1;
        let res = Solution::max_operations(input_vec, input_val);
        assert_eq!(output_val, res);
    }

}
