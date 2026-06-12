#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        s.retain(|s| s.is_alphabetic() || s.is_numeric());
        if s.len() <=1 {
            return true;
        }
        let s_lower = s.to_lowercase();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s_lower.as_bytes()[left] != s_lower.as_bytes()[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_str = "A man, a plan, a canal: Panama".to_string();
        let output_val = true;
        let res = Solution::is_palindrome(input_str);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_example_2() {
        let input_str = "race a car".to_string();
        let output_val = false;
        let res = Solution::is_palindrome(input_str);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_example_3() {
        let input_str = " ".to_string();
        let output_val = true;
        let res = Solution::is_palindrome(input_str);
        assert_eq!(output_val, res);
    }
}
