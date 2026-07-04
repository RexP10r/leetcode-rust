#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        intervals.sort_unstable_by_key(|interval| interval[0]);

        let mut res = Vec::with_capacity(intervals.len());
        let mut current = intervals[0].clone(); 

        for interval in intervals.into_iter().skip(1) {
            if current[1] >= interval[0] {
                current[1] = current[1].max(interval[1]);
            } else {
                res.push(current);
                current = interval;
            }
        }
        res.push(current);
        res
    }
}

#[cfg(test)]
mod test {
    use crate::problems::medium::lc56::Solution;

    #[test]
    fn test_example_1() {
        let input: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected: Vec<Vec<i32>> = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        let res = Solution::merge(input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_example_2() {
        let input: Vec<Vec<i32>> = vec![vec![1, 4], vec![4, 5]];
        let expected: Vec<Vec<i32>> = vec![vec![1, 5]];
        let res = Solution::merge(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_3() {
        let input: Vec<Vec<i32>> = vec![vec![4, 7], vec![1, 4]];
        let expected: Vec<Vec<i32>> = vec![vec![1, 7]];
        let res = Solution::merge(input);
        assert_eq!(expected, res);
    }
}
