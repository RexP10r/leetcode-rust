#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            return Self::intersect(nums2, nums1);
        }
        let mut nums1_map: HashMap<u16, u16> = HashMap::new();
        for n in nums1.into_iter() {
            nums1_map
                .entry(n as u16)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut res = Vec::new();
        for n2 in nums2.iter() {
            if let Some(n1_count) = nums1_map.get_mut(&(*n2 as u16))
                && *n1_count > 0 as u16
            {
                *n1_count = n1_count.saturating_sub(1);
                res.push(*n2);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::problems::easy::lc350::Solution;

    #[test]
    fn test_example_1() {
        let input1 = vec![1,2,2,1];
        let input2 = vec![2,2];
        let expected = vec![2,2];
        let res = Solution::intersect(input1, input2);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input1 = vec![4,9,5];
        let input2 = vec![9,4,9,8,4];
        let expected = vec![9, 4];
        let res = Solution::intersect(input1, input2);
        assert_eq!(expected, res);
    }
}

