#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k.try_into().unwrap();
        let mut nums_sum: i32 = nums.iter().take(k).sum();
        let mut max_sum = nums_sum;
        for i in k..=nums.len() - 1 {
            nums_sum = nums_sum - nums[i - k] + nums[i];
            if nums_sum > max_sum {
                max_sum = nums_sum;
            }
        }
        max_sum as f64 / k as f64
    }
}
#[cfg(test)]
mod test {
    use crate::problems::easy::lc643::Solution;

    #[test]
    fn test_example_1() {
        let input_vec = vec![1, 12, -5, -6, 50, 3];
        let input_num = 4;
        let expected: f64 = 12.75;
        let res = Solution::find_max_average(input_vec, input_num);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input_vec = vec![5];
        let input_num = 1;
        let expected: f64 = 5.0;
        let res = Solution::find_max_average(input_vec, input_num);
        assert_eq!(expected, res);
    }
}
