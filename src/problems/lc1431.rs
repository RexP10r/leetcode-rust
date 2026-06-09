#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies: i32 = *candies.iter().max().unwrap_or(&0);
        candies
            .into_iter()
            .map(|c| c + extra_candies >= max_candies)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_vec: Vec<i32> = vec![2, 3, 5, 1, 3];
        let input_value: i32 = 3;
        let output_vec: Vec<bool> = vec![true, true, true, false, true];
        let res = Solution::kids_with_candies(input_vec, input_value);
        assert_eq!(output_vec, res);
    }
    #[test]
    fn test_example_2() {
        let input_vec: Vec<i32> = vec![4, 2, 1, 1, 2];
        let input_value: i32 = 1;
        let output_vec: Vec<bool> = vec![true, false, false, false, false];
        let res = Solution::kids_with_candies(input_vec, input_value);
        assert_eq!(output_vec, res);
    }
    #[test]
    fn test_example_3() {
        let input_vec: Vec<i32> = vec![12,1,12];
        let input_value: i32 = 10;
        let output_vec: Vec<bool> = vec![true, false, true];
        let res = Solution::kids_with_candies(input_vec, input_value);
        assert_eq!(output_vec, res);
    }
}
