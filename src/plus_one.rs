//! https://leetcode.com/problems/plus-one/

pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
		let mut finished = false;
		for digit in digits.iter_mut().rev() {
			if *digit < 9 {
				*digit += 1;
				finished = true;
				break;
			}
			else {
				*digit = 0;
			}
		}

		if !finished {
			digits.insert(0, 1);
		}
		digits
    }
}