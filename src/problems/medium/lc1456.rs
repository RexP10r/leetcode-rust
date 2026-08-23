#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let is_vowel = |b: &u8| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u');
        let s = s.as_bytes();
        let k = k.try_into().unwrap();
        let mut cur_count = unsafe {
            s.get_unchecked(..k).iter().filter(|c| is_vowel(c)).count() as i32
        };
        let mut max_count = cur_count;
        for i in k..s.len() {
            unsafe {
                cur_count += is_vowel(&s.get_unchecked(i)) as i32;
                cur_count -= is_vowel(&s.get_unchecked(i-k)) as i32;
            }
            max_count = max_count.max(cur_count);
        }
        max_count
    }
}

#[cfg(test)]
mod test {
    use crate::problems::medium::lc1456::Solution;

    #[test]
    fn test_example_1() {
        let input_str = "abciiidef".to_string();
        let input_k = 3;
        let expected = 3;
        let res = Solution::max_vowels(input_str, input_k);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input_str = "aeiou".to_string();
        let input_k = 2;
        let expected = 2;
        let res = Solution::max_vowels(input_str, input_k);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_3() {
        let input_str = "leetcode".to_string();
        let input_k = 3;
        let expected = 2;
        let res = Solution::max_vowels(input_str, input_k);
        assert_eq!(expected, res);
    }
}
