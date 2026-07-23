#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let mut s_ptr = 0;
        let s_bytes = s.into_bytes();
        let t_bytes = t.into_bytes();
        let s_len = s_bytes.len();
        let t_len = t_bytes.len();
        for t_ptr in 0..t_len {
            unsafe {
                if t_bytes.get_unchecked(t_ptr) == s_bytes.get_unchecked(s_ptr) {
                    s_ptr += 1;
                }
            }
            if s_ptr == s_len {
                return true;
            }
        }
        false
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
