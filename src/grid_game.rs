//! https://leetcode.com/problems/grid-game/

pub struct Solution;

impl Solution {
    pub fn grid_game(mut grid: Vec<Vec<i32>>) -> i64 {
        println!("Grid:");
        for row in &grid {
            println!("{row:?}");
        }
        fn calc_sums(grid: &Vec<Vec<i32>>) -> (Vec<i64>, Vec<i64>) {
            let mut sums: (Vec<i64>, Vec<i64>) = (vec![0; grid[0].len()], vec![0; grid[0].len()]);

            sums.0[grid[0].len()-1] = grid[0].iter().copied().map(i64::from).sum();
            for i in (0..grid[0].len()-1).rev() {
                sums.0[i] = sums.0[i+1] - grid[0][i+1] as i64;
            }
            sums.1[0] = grid[1].iter().copied().map(i64::from).sum();
            for i in 1..grid[1].len() {
                sums.1[i] = sums.1[i-1] - grid[1][i-1] as i64;
            }

            
            println!("Grid sums:");
            println!("{:?}\n{:?}", sums.0, sums.1);
            

            return sums;
        }

        fn find_best_red_pivot(sums: &mut (Vec<i64>, Vec<i64>)) -> usize {
            
        }

        fn find_best_blue_pivot(sums: &(Vec<i64>, Vec<i64>)) -> (usize, (i64, i64)) {
            return sums.0.iter().copied().zip(sums.1.iter().copied())
                .enumerate()
                .max_by(|&(_, (a, b)), &(_, (c, d))| (a+b).cmp(&(c+d)))
                .unwrap();
        }
        
        let mut sums = calc_sums(&grid);
        let best_red = find_best_pivot(&sums);
        for (i, (sum1, sum2)) in sums.0.iter_mut().zip(sums.1.iter_mut()).enumerate() {
            if i < best_red.0 {
                grid[0][i] = 0;
                *sum1 = 0;
                *sum2 -= best_red.1.1;
            }
            else if i > best_red.0 {
                grid[1][i] = 0;
                *sum1 -= best_red.1.0;
                *sum2 = 0;
            }
            else {
                grid[0][i] = 0;
                grid[1][i] = 0;
                *sum1 = 0;
                *sum2 = 0;
            }
        }

        
        println!("Grid after Red:");
        for row in &grid {
            println!("{row:?}");
        }
        println!("Sums after Red:");
        println!("{:?}\n{:?}", sums.0, sums.1);
        

        let blue_sums = find_best_pivot(&sums).1;
        return blue_sums.0 + blue_sums.1;
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
}