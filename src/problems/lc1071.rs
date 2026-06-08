#![allow(dead_code)]
use crate::utils::gcd::euclidean_algo;
struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1
            .chars()
            .chain(str2.chars())
            .ne(str2.chars().chain(str1.chars()))
        {
            return "".to_string();
        }
        str1[0..euclidean_algo(str1.len(), str2.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();
        let reference = "ABC".to_string();
        let answer = Solution::gcd_of_strings(str1, str2);
        assert_eq!(reference, answer);
    }

    #[test]
    fn test_example_2() {
        let str1 = "ABABAB".to_string();
        let str2 = "AB".to_string();
        let reference = "AB".to_string();
        let answer = Solution::gcd_of_strings(str1, str2);
        assert_eq!(reference, answer);
    }
    #[test]
    fn test_example_3() {
        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();
        let reference = "".to_string();
        let answer = Solution::gcd_of_strings(str1, str2);
        assert_eq!(reference, answer);
    }
    #[test]
    fn test_example_4() {
        let str1 = "AAAAAB".to_string();
        let str2 = "AAA".to_string();
        let reference = "".to_string();
        let answer = Solution::gcd_of_strings(str1, str2);
        assert_eq!(reference, answer);
    }
}
