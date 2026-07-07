#![allow(dead_code)]
struct Solution;
impl Solution {
    fn intersection(left: &Vec<i32>, right: &Vec<i32>) -> Option<Vec<i32>> {
        let max_of_starts = left[0].max(right[0]);
        let min_of_ends = left[1].min(right[1]);
        if max_of_starts <= min_of_ends {
            Some(vec![max_of_starts, min_of_ends])
        } else {
            None
        }
    }
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut first_p = 0;
        let mut second_p = 0;
        while first_p < first_list.len() && second_p < second_list.len() {
            if let Some(intersection) =
                Self::intersection(&first_list[first_p], &second_list[second_p])
            {
                res.push(intersection);
            }
            if first_list[first_p][1] <= second_list[second_p][1] {
                first_p += 1;
            } else {
                second_p += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use crate::problems::medium::lc986::Solution;

    #[test]
    fn test_example_1() {
        let input_1 = vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]];
        let input_2 = vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]];
        let expected = vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25],
        ];
        let res = Solution::interval_intersection(input_1, input_2);
        assert_eq!(expected, res);
    }
}
