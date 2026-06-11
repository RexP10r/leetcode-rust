#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 0;
        let length = chars.len();
        while right < length {
            let mut count = 0;
            {
                let symbol = chars[right];
                while right < length && chars[right] == symbol {
                    right += 1;
                    count += 1;
                }
                chars[left] = symbol;
            }
            left += 1;
            if count > 1 {
                for c in count.to_string().chars() {
                    chars[left] = c;
                    left += 1;
                }
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut input_vec = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let output_val = 6; //'a2b2c3"
        let res = Solution::compress(&mut input_vec);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_example_2() {
        let mut input_vec = vec!['a'];
        let output_val = 1; //'a'
        let res = Solution::compress(&mut input_vec);
        assert_eq!(output_val, res);
    }
    #[test]
    fn test_example_3() {
        let mut input_vec = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let output_val = 4; //'ab12'
        let res = Solution::compress(&mut input_vec);
        assert_eq!(output_val, res);
    }
}
