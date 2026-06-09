#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut words = s.split_whitespace().rev();
        if let Some(first_word) = words.next() {
            res.push_str(first_word);
        }
        for word in words {
            res.push(' ');
            res.push_str(word);

        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_str = "the sky is blue".to_string();
        let output_vec = "blue is sky the".to_string();
        let res = Solution::reverse_words(input_str);
        assert_eq!(output_vec, res);
    }
    #[test]
    fn test_example_2() {
        let input_str = "  hello world  ".to_string();
        let output_vec = "world hello".to_string();
        let res = Solution::reverse_words(input_str);
        assert_eq!(output_vec, res);
    }
    #[test]
    fn test_example_3() {
        let input_str = "a good  example".to_string();
        let output_vec = "example good a".to_string();
        let res = Solution::reverse_words(input_str);
        assert_eq!(output_vec, res);
    }
}
