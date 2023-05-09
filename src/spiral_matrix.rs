//! https://leetcode.com/problems/spiral-matrix/

pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }

        let mut min_x = 0;
        let mut max_x = matrix[0].len();
        let mut min_y = 0;
        let mut max_y = matrix.len();

        let mut numbers = Vec::with_capacity(matrix.len() * matrix[0].len());

        #[repr(u8)]
        #[derive(Clone, Copy, Debug)]
        enum Direction {
            Right,
            Down,
            Left,
            Up,
        }

        impl Direction {
            fn next(&mut self) {
                use Direction::*;
                *self = match self {
                    Right => Down,
                    Down => Left,
                    Left => Up,
                    Up => Right,
                }
            }
        }

        let mut direction = Direction::Right;
        while min_x != max_x && min_y != max_y {
            match direction {
                Direction::Right => {
                    numbers.extend_from_slice(&matrix[min_y][min_x..max_x]);
                    min_y += 1;
                },
                Direction::Down => {
                    numbers.extend(matrix[min_y..max_y].iter().map(|vec| vec[max_x-1]));
                    max_x -= 1;
                },
                Direction::Left => {
                    numbers.extend(matrix[max_y-1][min_x..max_x].iter().rev());
                    max_y -= 1;
                },
                Direction::Up => {
                    numbers.extend(matrix[min_y..max_y].iter().rev().map(|vec| vec[min_x]));
                    min_x += 1;
                },
            }
            direction.next();
        }

        numbers
    }
}

#[test]
fn test() {
    fn test_spiral(matrix: &[&[i32]], expected: &[i32]) {
        let matrix_vec = matrix.into_iter().map(|row| row.to_vec()).collect();
        let result = Solution::spiral_order(matrix_vec);
        for row in matrix {
            println!("{row:?}");
        }
        println!("Result: {result:?} (Expected {expected:?})");
        assert_eq!(&result, expected);
    }

    test_spiral(&[
        &[1, 2, 3],
        &[4, 5, 6],
        &[7, 8, 9],
    ], &[1, 2, 3, 6, 9, 8, 7, 4, 5]);
    test_spiral(&[
        &[1,2,3,4],
        &[5,6,7,8],
        &[9,10,11,12],
    ], &[1,2,3,4,8,12,11,10,9,5,6,7]);
    test_spiral(&[
        &[5]
    ], &[5]);
    test_spiral(&[
        &[1,2,3,4],
        &[5,6,7,8],
        &[9,10,11,12],
        &[13,14,15,16],
    ], &[1,2,3,4,8,12,16,15,14,13,9,5,6,7,11,10]);
}