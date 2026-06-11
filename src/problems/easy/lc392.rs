#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut it = t.chars();
        s.chars().into_iter().all(|sc| it.any(|tc| tc == sc))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_substr = "abc".to_string();
        let input_str = "ahbgdc".to_string();
        let output_val = true;
        assert_eq!(
            Solution::is_subsequence(input_substr, input_str),
            output_val
        );
    }
    #[test]
    fn test_example_2() {
        let input_substr = "axc".to_string();
        let input_str = "ahbgdc".to_string();
        let output_val = false;
        assert_eq!(
            Solution::is_subsequence(input_substr, input_str),
            output_val
        );
    }
    #[test]
    fn test_example_3() {
        let input_substr = "acb".to_string();
        let input_str = "ahbgdc".to_string();
        let output_val = false;
        assert_eq!(
            Solution::is_subsequence(input_substr, input_str),
            output_val
        );
    }
}
