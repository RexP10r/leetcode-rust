#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());

        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        for (c1, c2) in chars1.by_ref().zip(chars2.by_ref()) {
            result.push(c1);
            result.push(c2);
        }

        result.extend(chars1);
        result.extend(chars2);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exmaple_1() {
        let word1 = String::from("abc");
        let word2 = String::from("dfh");
        let reference = String::from("adbfch");
        assert_eq!(Solution::merge_alternately(word1, word2), reference);
    }
    #[test]
    fn test_exmaple_2() {
        let word1 = String::from("abc");
        let word2 = String::from("dfhd");
        let reference = String::from("adbfchd");
        assert_eq!(Solution::merge_alternately(word1, word2), reference);
    }
    #[test]
    fn test_exmaple_3() {
        let word1 = String::from("abcm");
        let word2 = String::from("df");
        let reference = String::from("adbfcm");
        assert_eq!(Solution::merge_alternately(word1, word2), reference);
    }
}
