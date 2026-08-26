#![allow(dead_code)]
struct Solution;
impl Solution {
    fn get_freq(word: &[u8]) -> Vec<u32> {
        word.iter().fold(vec![0; 26], |mut freq, symbol| {
            freq[(symbol - b'a') as usize] += 1;
            freq
        })
    }
    fn clean_freq(freq: Vec<u32>) -> Vec<u32> {
        freq.into_iter().filter(|x| *x>0).collect()
    }
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let freq1 = Self::get_freq(word1.as_bytes());
        let freq2 = Self::get_freq(word2.as_bytes());
        for i in 0..26 {
            if (freq1[i] == 0) != (freq2[i] == 0) {
                return false;
            }
        }
        let mut count1 = Self::clean_freq(freq1);
        let mut count2 = Self::clean_freq(freq2);
        count1.sort();
        count2.sort();
        count1 == count2
    }
}
