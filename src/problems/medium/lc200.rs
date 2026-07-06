#![allow(dead_code)]

struct Solution;

impl Solution {
    fn is_valid_point(grid: &Vec<Vec<char>>, i: &i32, j: &i32) -> bool {
        *i >= 0 && *j >= 0 && (*i as usize) < grid.len() && (*j as usize) < grid[0].len()
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: &usize, j: &usize, directions: &Vec<(i32, i32)>) {
        for (id, jd) in directions {
            let check_i = *i as i32 - id;
            let check_j = *j as i32 - jd;
            if !Self::is_valid_point(grid, &check_i, &check_j) {
                continue;
            }
            let cur_i = check_i as usize;
            let cur_j = check_j as usize;
            if grid[cur_i][cur_j] == '1' {
                grid[cur_i][cur_j] = '0';
                Self::dfs(grid, &cur_i, &cur_j, directions);
            }
        }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let directions = &vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    grid[i][j] = '0';
                    Self::dfs(&mut grid, &i, &j, directions);
                    count += 1;
                }
            }
        }
        count
    }
}
