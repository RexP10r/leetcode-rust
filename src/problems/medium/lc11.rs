#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_field = 0;
        while left < right {
            {
                let current_field = height[left].min(height[right]) * (right - left) as i32;
                max_field = max_field.max(current_field);
            }

            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_field
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_vec = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output_val = 49;
        let res = Solution::max_area(input_vec);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_example_2() {
        let input_vec = vec![1, 2, 4, 3];
        let output_val = 4;
        let res = Solution::max_area(input_vec);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_example_3() {
        let input_vec = vec![1, 1];
        let output_val = 1;
        let res = Solution::max_area(input_vec);
        assert_eq!(output_val, res);
    }
}
