#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        for &c in s.as_bytes() {
            match c {
                b'(' | b'[' | b'{' => stack.push(c),
                b')' => {
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "()".to_string();
        let output = true;
        let res = Solution::is_valid(input);
        assert_eq!(output, res);
    }
    #[test]
    fn test_example_2() {
        let input = "()[]{}".to_string();
        let output = true;
        let res = Solution::is_valid(input);
        assert_eq!(output, res);
    }
    #[test]
    fn test_example_3() {
        let input = "(}".to_string();
        let output = false;
        let res = Solution::is_valid(input);
        assert_eq!(output, res);
    }
    #[test]
    fn test_example_4() {
        let input = "([])".to_string();
        let output = true;
        let res = Solution::is_valid(input);
        assert_eq!(output, res);
    }
    #[test]
    fn test_example_5() {
        let input = "([)]".to_string();
        let output = false;
        let res = Solution::is_valid(input);
        assert_eq!(output, res);
    }
}
