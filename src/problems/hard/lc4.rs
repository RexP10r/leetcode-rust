#![allow(dead_code)]

struct Solution;
impl Solution {
    fn merge_sorted<T: Ord>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
        let mut i1 = v1.into_iter().peekable();
        let mut i2 = v2.into_iter().peekable();
        let mut merged: Vec<T> = Vec::with_capacity(i1.size_hint().0 + i2.size_hint().0);
        while let (Some(val1), Some(val2)) = (i1.peek(), i2.peek()) {
            if val1 <= val2 {
                if let Some(v) = i1.next() {
                    merged.push(v);
                }
            } else {
                if let Some(v) = i2.next() {
                    merged.push(v);
                }
            }
        }

        merged.extend(i1);
        merged.extend(i2);
        merged
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged = Self::merge_sorted(nums1, nums2);
        let length = merged.len();
        let mid = length / 2;
        match length % 2 == 0 {
            true => (merged[mid - 1] + merged[mid]) as f64 / 2.0,
            false => merged[mid] as f64,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::problems::hard::lc4::Solution;

    #[test]
    fn test_example_1() {
        let input_1 = vec![1, 3];
        let input_2 = vec![2];
        let expected = 2.0;
        let res = Solution::find_median_sorted_arrays(input_1, input_2);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_example_2() {
        let input_1 = vec![1, 2];
        let input_2 = vec![3, 4];
        let expected = 2.5;
        let res = Solution::find_median_sorted_arrays(input_1, input_2);
        assert_eq!(expected, res);
    }
}
