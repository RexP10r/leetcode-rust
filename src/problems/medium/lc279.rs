#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let mut dp: Vec<u16> = Vec::new();
        dp.push(0);
        for _ in 0..n {
            let m = dp.len();
            let mut count_sqrs = u16::MAX;
            let mut i = 1;
            while i * i <= m {
                count_sqrs = count_sqrs.min(dp[m - i * i] + 1);
                i += 1;
            }
            dp.push(count_sqrs);
        }
        dp[n as usize] as i32
    }
}


#[cfg(test)]
mod test {
    use crate::problems::medium::lc279::Solution;

    #[test]
    fn test_example_1(){
        let input = 12;
        let expected = 3;
        let res = Solution::num_squares(input);
        assert_eq!(expected, res)
    }
    #[test]
    fn test_example_2(){
        let input = 13;
        let expected = 2;
        let res = Solution::num_squares(input);
        assert_eq!(expected, res)
    }
}
