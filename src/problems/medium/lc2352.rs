#![allow(dead_code)]

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let hm = grid
            .iter()
            .fold(HashMap::<_, u8>::new(), |mut hm, row| {
                *hm.entry(row.to_vec()).or_insert(0) += 1;
                hm
            });
        (0..grid[0].len()).map(|col_idx| {
            let col = grid.iter().map(|row| row[col_idx]).collect::<Vec<_>>();
            hm.get(&col).copied().unwrap_or_default() as i32
        }).sum()
    }
}
