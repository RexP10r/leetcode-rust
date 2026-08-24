#![allow(dead_code)]

struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<_> = nums1.into_iter().collect();
        let set2: HashSet<_> = nums2.into_iter().collect();
        vec![
            set1.iter()
                .copied()
                .filter(|s1| !set2.contains(&s1))
                .collect(),
            set2.iter()
                .copied()
                .filter(|s2| !set1.contains(&s2))
                .collect(),
        ]
    }
}
