//! https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/

pub struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
		// Note: Screw whoever solved this in Python.
		// Python ints grow infinitely large and make this EASY.
		const MOD: u32 = 1000000007;
        nums.sort_unstable();

		fn binary_search_highest(nums: &[i32], target: i32) -> Option<usize> {
			use std::cmp::Ordering::*;
			if nums.len() == 0 {
				return None;
			}
			else if nums.len() == 1 {
				if nums[0] <= target {
					return Some(0);
				}
				else {
					return None;
				}
			}
			let mut mid = nums.len() / 2;
			match target.cmp(&nums[mid]) {
				Greater => {
					return binary_search_highest(&nums[mid..], target).map(|i| i+mid);
				},
				Less => {
					return binary_search_highest(&nums[..mid], target);
				},
				Equal => {
					while mid < nums.len()-1 && nums[mid+1] == target {
						mid += 1;
					}
					return Some(mid);
				},
			}
		}

		fn count_subsequences(nums: &[i32], target: i32) -> u32 {
			let end = binary_search_highest(nums, target-nums[0]);

			const MAX_SHIFT: u8 = 98;

			if let Some(mut end) = end {
				let mut result: u128 = 1;
				while end > 0 {
					let shift = end.min(MAX_SHIFT as usize);
					result = (result << shift) % MOD as u128;
					end -= shift;
				}
				return result as u32;
			}
			else {
				return 0;
			}
		}

		let mut sequences = 0;

		let highest = binary_search_highest(&nums, target-nums[0]);
		if highest.is_none() {
			return 0;
		}

		for i in 0..=highest.unwrap() {
			let next_sequences = count_subsequences(&nums[i..], target);
			if next_sequences == 0 {
				break;
			}
			sequences = (sequences + next_sequences) % MOD;
		}

		return sequences as i32;
    }
}

#[test]
fn test() {
	fn test_subsequences(nums: &[i32], target: i32, expected: i32) {
		let result = Solution::num_subseq(nums.to_vec(), target);
		println!("{{len {}}} target {target} = {result} (Expected {expected})", nums.len());
		assert_eq!(result, expected);
	}

	test_subsequences(&[3,5,6,7], 9, 4);
	test_subsequences(&[3,3,6,8], 10, 6);
	test_subsequences(&[2,3,3,4,6,7], 12, 61);
	test_subsequences(
		&[14,4,6,6,20,8,5,6,8,12,6,10,14,9,17,16,9,7,14,11,14,15,13,11,10,18,13,17,17,14,17,7,9,5,10,13,8,5,18,20,7,5,5,15,19,14],
		22,
		272187084,
	);
	test_subsequences(
		&[9,25,9,28,24,12,17,8,28,7,21,25,10,2,16,19,12,13,15,28,14,12,24,9,6,7,2,15,19,13,30,30,23,19,11,3,17,2,14,20,22,30,12,1,11,2,2,20,20,27,15,9,10,4,12,30,13,5,2,11,29,5,3,13,22,5,16,19,7,19,11,16,11,25,29,21,29,3,2,9,20,15,9],
		32,
		91931447,
	);
	test_subsequences(
		&[82,30,56,28,63,24,17,50,45,95,50,41,10,59,59,17,39,11,36,64,44,16,56,31,83,100,12,69,13,46,58,92,55,71,33,99,78,13,17,50,82,33,12,22,48,48,77,71,11,83,26,51,26,56,51,82,54,50,13,64,83,48,31,28,33,89,60,22,25,35,89,80,65,92,52,29,64,96,98,76,93,77,90,72,49,62,78,41,99,22,36,64,23,76,71,69,83,84,77,72,24,21,89,37,20,59,48,78,68,60,25,93,38,83,16,85,10,20,34,98,73,16,45,99,75,29,12,27,35,54,72,10,99,61,92,67,43,31,27,88,51,92,75,59,47,48,48,22,61,69,69,90,61,85,48,83,79,96,27,48,24,20,35,52,49,58,84,71,30,89,72,74,72,87,31,43,66,18,92,63,48,71,75,42,37,53,76,79,87],
		125,
		945569954,
	);
	test_subsequences(
		include!("large_test.txt"),
		119238,
		905345521,
	);
	test_subsequences(&[5,2,4,1,7,6,8], 16, 127);
}