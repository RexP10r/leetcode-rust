#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left_p, mut left_max, mut right_p, mut right_max, mut res) =
            (0, 0, height.len() - 1, 0, 0);
        while left_p < right_p {
            left_max = left_max.max(height[left_p]);
            right_max = right_max.max(height[right_p]);
            if left_max < right_max {
                res += left_max - height[left_p];
                left_p += 1;
            } else {
                res += right_max - height[right_p];
                right_p -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::problems::hard::lc42::Solution;

    #[test]
    fn test_example_1(){
        let input = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let expected = 6;
        let res = Solution::trap(input);
        assert_eq!(expected, res);
    }
}
