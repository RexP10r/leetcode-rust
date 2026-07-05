#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&idx) = map.get(&(target - num)) {
                return vec![idx as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}
#[cfg(test)]
mod test {
    use crate::problems::easy::lc1::Solution;

    #[test]
    fn test_example_1() {
        let input_vec = vec![2, 7, 11, 15];
        let input_num = 9;
        let expected = vec![0, 1];
        let res = Solution::two_sum(input_vec, input_num);
        assert_eq!(expected, res);
    }
}
