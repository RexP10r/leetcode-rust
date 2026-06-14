#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut mp: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len()); 
        for s in strs {
            let mut count = [0u8; 26];
            for c in s.as_bytes() {
                count[(c - b'a') as usize] += 1;
            }
            mp.entry(count).or_default().push(s);
        };
        mp.into_iter().map(|(_,strs)| strs).collect()
    }
}
