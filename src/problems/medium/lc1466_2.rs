#![allow(dead_code)]

struct Solution;
use std::iter;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let adj = connections.into_iter().fold(
            {
                let empty_vecs = iter::repeat(vec![]).take(n);
                Vec::from_iter(empty_vecs)
            },
            |mut adj, conn| {
                adj[conn[0] as usize].push(conn[1]);
                adj[conn[1] as usize].push(-conn[0]);
                adj
            },
        );
        let mut visited = vec![false; n];
        let mut count = 0;

        let mut stack = Vec::new();
        stack.push(0);
        visited[0] = true;

        while let Some(src) = stack.pop() {
            visited[src] = true;
            for &dest in &adj[src] {
                let next_node = dest.abs() as usize;
                if !visited[next_node] {
                    count += (dest > 0) as i32;
                    stack.push(next_node);
                }
            }
            
        }
        count
    }
}
#[cfg(test)]
mod test {
    use crate::problems::medium::lc1466_2::Solution;

    #[test]
    fn test_example_1() {
        let n_nodes = 6;
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
        let expected = 3;
        let res = Solution::min_reorder(n_nodes, connections);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_example_2() {
        let n_nodes = 5;
        let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
        let expected = 2;
        let res = Solution::min_reorder(n_nodes, connections);
        assert_eq!(expected, res);
    }
}
