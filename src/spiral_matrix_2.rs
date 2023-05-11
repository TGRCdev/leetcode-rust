pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
		let n = n as usize;

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

		let mut matrix = vec![vec![0; n]; n];
		let mut min_x = 0;
		let mut max_x = n;
		let mut min_y = 0;
		let mut max_y = n;

		let mut direction = Direction::Right;
		let mut i = 1;
		while min_x != max_x && min_y != max_y {
            match direction {
                Direction::Right => {
                    matrix[min_y][min_x..max_x].iter_mut().for_each(|elem| {
						let num = i;
						i += 1;
						*elem = num;
					});
                    min_y += 1;
                },
                Direction::Down => {
                    matrix[min_y..max_y].iter_mut().for_each(|row| {
						let num = i;
						i += 1;
						row[max_x-1] = num;
					});
                    max_x -= 1;
                },
                Direction::Left => {
                    matrix[max_y-1][min_x..max_x].iter_mut().rev().for_each(|elem| {
						let num = i;
						i += 1;
						*elem = num;
					});
                    max_y -= 1;
                },
                Direction::Up => {
                    matrix[min_y..max_y].iter_mut().rev().for_each(|row| {
						let num = i;
						i += 1;
						row[min_x] = num;
					});
                    min_x += 1;
                },
            }
            direction.next();
        }

		matrix
    }
}

#[test]
fn test() {
	fn test_matrix(n: i32, expected: &[&[i32]]) {
		let result = Solution::generate_matrix(n);
		println!("Matrix {n} - Expected");
		for (row1, row2) in result.iter().zip(expected) {
			println!("{row1:?}\t{row2:?}");
		}
		assert_eq!(result, expected);
	}

	test_matrix(3, &[&[1,2,3],&[8,9,4],&[7,6,5]]);
}