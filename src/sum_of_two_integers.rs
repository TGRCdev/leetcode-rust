//! https://leetcode.com/problems/sum-of-two-integers/

pub struct Solution;

impl Solution {
    pub fn get_sum(mut a: i32, b: i32) -> i32 {
		let mut carry = false;
		for bit in (0..32).map(|bit| 1 << bit) {
			match (a & bit != 0) as u8 + (b & bit != 0) as u8 + carry as u8 {
				3 | 0 => (),
				2 => {
					carry = true;
					a &= !bit;
				},
				1 => {
					carry = false;
					a |= bit;
				}
				_ => unreachable!(),
			}
		}

		return a;
    }
}

#[test]
fn random_test() {
	use rand::prelude::*;
	const TEST_COUNT: usize = 1000;
	for i in 0..TEST_COUNT {
		let (mut a, mut b): (i32, i32) = (random(), random());
		while a.checked_add(b).is_none() {
			(a, b) = (random(), random())
		}
		let expected = a + b;
		let result = Solution::get_sum(a, b);
		println!("Test {}: {a} + {b} = {result} (Expected {expected})", i+1);
		assert_eq!(result, expected);
	}
}

#[test]
fn test() {
	fn test_add(a: i32, b: i32) {
		let expected = a + b;
		let result = Solution::get_sum(a, b);
		println!("{a} + {b} = {result} (Expected {expected})");
		assert_eq!(result, expected);
	}

	test_add(7,7);
	
}