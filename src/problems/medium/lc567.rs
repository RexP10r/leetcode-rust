#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        if s2.len() < s1.len() {
            return false;
        }

        let mut s1_count: HashMap<u8, u8> = HashMap::with_capacity(s1.len());
        let mut window_count: HashMap<u8,u8> = HashMap::with_capacity(s1.len());

        for i in 0..s1.len() {
            *s1_count.entry(s1_bytes[i]).or_insert(0) += 1;
            *window_count.entry(s2_bytes[i]).or_insert(0) += 1;
        }
        if s1_count == window_count {
            return true;
        }
        for right in s1.len()..s2.len() {
            let new_char = s2_bytes[right];
            *window_count.entry(new_char).or_insert(0) += 1;
            
            let old_char = s2_bytes[right-s1.len()];
            if let Some(count) = window_count.get_mut(&old_char) {
                if *count == 1 {
                    window_count.remove(&old_char);
                } else {
                    *count -= 1;
                }
            }
            if window_count == s1_count {
                return true;
            }

        }
        false
    }
}
