#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn can_place_flowers(mut arr: Vec<i32>, mut n: i32) -> bool {
        let length = arr.len();
        for i in 0..length {
            let left_right = (i == 0 || arr[i - 1] == 0) && (i == length - 1 || arr[i + 1] == 0);

            if left_right && arr[i] == 0 {
                arr[i] = 1;
                n -= 1;
            }
        }
        n <= 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_vec = vec![1, 0, 0, 0, 1];
        let input_val = 1;
        let output_val = true;
        let res = Solution::can_place_flowers(input_vec, input_val);
        assert_eq!(res, output_val);
    }

    #[test]
    fn test_example_2() {
        let input_vec = vec![1, 0, 0, 0, 1];
        let input_val = 2;
        let output_val = false;
        let res = Solution::can_place_flowers(input_vec, input_val);
        assert_eq!(res, output_val);
    }
    #[test]
    fn test_example_3() {
        let input_vec = vec![0, 0, 1, 0, 1];
        let input_val = 1;
        let output_val = true;
        let res = Solution::can_place_flowers(input_vec, input_val);
        assert_eq!(res, output_val);
    }
    #[test]
    fn test_example_4() {
        let input_vec = vec![1, 0, 1, 0, 0];
        let input_val = 1;
        let output_val = true;
        let res = Solution::can_place_flowers(input_vec, input_val);
        assert_eq!(res, output_val);
    }
    #[test]
    fn test_example_5() {
        let input_vec = vec![0, 0, 1, 0, 0];
        let input_val = 1;
        let output_val = true;
        let res = Solution::can_place_flowers(input_vec, input_val);
        assert_eq!(res, output_val);
    }
}
