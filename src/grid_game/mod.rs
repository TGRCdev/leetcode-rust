//! https://leetcode.com/problems/grid-game/

pub struct Solution;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        
        let mut left: i64 = grid[0][1..].iter().map(|i| *i as i64).sum();
        let mut right: i64 = 0;
        let mut lowest = left.max(right);

        let grid_len = grid[0].len();

        for (num1, num2) in grid[0][1..].iter().zip(&grid[1][..grid_len-1]) {
            left -= *num1 as i64;
            right += *num2 as i64;
            lowest = lowest.min(left.max(right));
        };

        return lowest;
    }
}

#[test]
fn test() {
    fn test_grid(grid: [&[i32]; 2], expected: i64) {
        let grid_vec = grid.into_iter().map(|g| g.to_vec()).collect();
        let result = Solution::grid_game(grid_vec);
        println!("{grid:?} = {result} (Expected {expected})");
        assert_eq!(result, expected);
    }

    test_grid([&[2,5,4],&[1,5,1]], 4);
    test_grid([&[3,3,1],&[8,5,2]], 4);
    test_grid([&[1,3,1,15],&[1,3,3,1]], 7);
    test_grid([&[2,5,1,4,4],&[1,5,1,1,1]], 7);
    test_grid(include!("long_testcase.txt"), 2374739160);
}