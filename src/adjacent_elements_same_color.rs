//! https://leetcode.com/problems/number-of-adjacent-elements-with-the-same-color
//! Solved during Weekly Contest 344 https://leetcode.com/contest/weekly-contest-344/

pub struct Solution;

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        if n <= 1 {
			return vec![0; queries.len()];
		}

		let mut colors = vec![0;n as usize];
		let mut adjacent_count = 0;
		let mut adj_vec = Vec::with_capacity(queries.len());
		
		enum Change {
			Gained,
			Lost,
			NoChange
		}
		use Change::*;

		for query in queries {
			let index = query[0] as usize;
			let color = query[1];
			if color != colors[index] {
				let left = {
					if index > 0 && !(colors[index] == 0 && colors[index-1] == 0) {
						match (colors[index-1] == colors[index], colors[index-1] == color) {
							(true, true) | (false, false) => Change::NoChange,
							(false, true) => Change::Gained,
							(true, false) => Change::Lost,
						}
					}
					else {
						Change::NoChange
					}
				};
				let right = {
					if index < (n-1) as usize && !(colors[index] == 0 && colors[index+1] == 0) {
						match (colors[index+1] == colors[index], colors[index+1] == color) {
							(true, true) | (false, false) => Change::NoChange,
							(false, true) => Change::Gained,
							(true, false) => Change::Lost,
						}
					}
					else {
						Change::NoChange
					}
				};
				match left {
					Gained => adjacent_count += 1,
					Lost => adjacent_count -= 1,
					NoChange => (),
				}
				match right {
					Gained => adjacent_count += 1,
					Lost => adjacent_count -= 1,
					NoChange => (),
				}
	
				colors[index] = color;
			}
			adj_vec.push(adjacent_count);
		}

		return adj_vec;
    }
}