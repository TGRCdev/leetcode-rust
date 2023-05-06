//! https://leetcode.com/problems/grid-game/

pub struct Solution;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        /*
        println!("Before");
        println!("{:?}\n{:?}", grid[0], grid[1]);
        */

        let grid_len = grid[0].len();

        let mut sums_after_red = (
            vec![0i64; grid_len],
            vec![0i64; grid_len],
        );

        for i in (0..grid_len-1).rev() {
            sums_after_red.0[i] = sums_after_red.0[i+1] + grid[0][i+1] as i64;
        }

        for i in 1..grid_len {
            sums_after_red.1[i] = sums_after_red.1[i-1] + grid[1][i-1] as i64;
        }
    
        /*
        println!("After");
        println!("{:?}\n{:?}", sums_after_red.0, sums_after_red.1);
        */

        return sums_after_red.0.into_iter()
            .zip(sums_after_red.1.into_iter())
            .map(|(topsum, bottomsum)| topsum.max(bottomsum) as i64)
            .min()
            .unwrap();
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