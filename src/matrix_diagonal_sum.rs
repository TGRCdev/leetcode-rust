//! https://leetcode.com/problems/matrix-diagonal-sum/

pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
		let mut sum = 0;
        let mid = mat[0].len() / 2;
		for i in 0..mat[0].len() {
			sum += mat[i][i];
			if mid != i || mat[0].len() % 2 == 0 {
				sum += mat[i][mat[0].len()-(1+i)];
			}
			
		}

		sum
    }
}