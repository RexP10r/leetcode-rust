#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut new_nums = vec![0; nums.len()];
        let mut left = 0;
        let mut right = nums.len() - 1;
        for k in (0..nums.len()).rev() {
            new_nums[k] = if nums[left].abs() >= nums[right].abs() {
                left += 1;
                nums[left - 1].pow(2)
            } else {
                right -= 1;
                nums[right + 1].pow(2)
            }
        }
        new_nums
    }
}
#[cfg(test)]
mod test {
    use crate::problems::easy::lc977::Solution;

    #[test]
    fn test_example_1() {
        let input = vec![-4, -1, 0, 3, 10];
        let expected = vec![0, 1, 9, 16, 100];
        let res = Solution::sorted_squares(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input = vec![-7, -3, 2, 3, 11];
        let expected = vec![4, 9, 9, 49, 121];
        let res = Solution::sorted_squares(input);
        assert_eq!(expected, res);
    }
}
