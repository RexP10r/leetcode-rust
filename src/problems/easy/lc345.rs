#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut right = s.len().saturating_sub(1);
        let mut left: usize = 0;

        let mut bytes = s.into_bytes();
        let is_vowel = |b: u8| {
            matches!(
                b,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        };

        while left < right {
            if !is_vowel(bytes[left]) {
                left += 1;
                continue;
            }
            if !is_vowel(bytes[right]) {
                right -= 1;
                continue;
            }
            bytes.swap(left, right);
            left += 1;
            right -= 1;
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_str = "IceCreAm".to_string();
        let output_str = "AceCreIm".to_string();
        let res = Solution::reverse_vowels(input_str);
        assert_eq!(output_str, res);
    }
    #[test]
    fn test_example_2() {
        let input_str = "leetcode".to_string();
        let output_str = "leotcede".to_string();
        let res = Solution::reverse_vowels(input_str);
        assert_eq!(output_str, res);
    }
}
