//! https://leetcode.com/problems/unique-paths/

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        if m == 1 || n == 1 { return 1; }

        let mut grid_paths = vec![1; m];

        for _ in 0..n-1 {
            for i in (0..m-1).rev() {
                grid_paths[i] += grid_paths[i+1];
            }
        }

        return grid_paths[0];
    }
}