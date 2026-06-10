#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for num in nums {
            match num {
                a if a <= first => first = num,
                a if a <= second => second = num,
                _ => return true,
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_vec = vec![1, 2, 3, 4, 5];
        let output_val = true;
        let res = Solution::increasing_triplet(input_vec);
        assert_eq!(output_val, res);
    }

    #[test]
    fn test_example_2() {
        let input_vec = vec![5, 4, 3, 2, 1];
        let output_val = false;
        let res = Solution::increasing_triplet(input_vec);
        assert_eq!(output_val, res);
    }

    #[test]
    fn test_example_3() {
        let input_vec = vec![2, 1, 5, 0, 4, 6];
        let output_val = true;
        let res = Solution::increasing_triplet(input_vec);
        assert_eq!(output_val, res);
    }
}
