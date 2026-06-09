#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        {
            let mut left_prod = 1;
            for i in 0..n {
                res[i] = left_prod;
                left_prod *= nums[i];
            }
        }

        {
            let mut right_prod = 1;
            for i in (0..n).rev() {
                res[i] *= right_prod;
                right_prod *= nums[i];
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_vec = vec![1, 2, 3, 4];
        let output_vec = vec![24, 12, 8, 6];
        let res = Solution::product_except_self(input_vec);
        assert_eq!(output_vec, res);
    }
    #[test]
    fn test_example_2() {
        let input_vec = vec![-1, 1, 0, 3, -3];
        let output_vec = vec![0, 0, 9, 0, 0];
        let res = Solution::product_except_self(input_vec);
        assert_eq!(output_vec, res);
    }
}
