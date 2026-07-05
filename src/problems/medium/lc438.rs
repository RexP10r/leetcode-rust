#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }
        let s_bytes = s.into_bytes();
        let p_bytes = p.into_bytes();
        let mut res: Vec<i32> = Vec::new();

        let mut target = [0u8; 26];
        let mut current = target.clone();
        for i in 0..p_bytes.len() {
            target[(p_bytes[i] - b'a') as usize] += 1;
            current[(s_bytes[i] - b'a') as usize] += 1;
        }
        if current == target {
            res.push(0);
        }

        for right in p_bytes.len()..s_bytes.len() {
            let left = right - p_bytes.len();
            current[(s_bytes[left] - b'a') as usize] -= 1;
            current[(s_bytes[right] - b'a') as usize] += 1;

            if current == target {
                res.push((left + 1) as i32);
            }
        }

        res
    }
}
#[cfg(test)]
mod test {
    use crate::problems::medium::lc438::Solution;

    #[test]
    fn test_example_1() {
        let input_s = "cbaebabacd".to_string();
        let input_p = "abc".to_string();
        let expected = vec![0, 6];
        let res = Solution::find_anagrams(input_s, input_p);
        assert_eq!(expected, res);
    }
}
