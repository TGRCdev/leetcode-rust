//! https://leetcode.com/problems/longest-increasing-subsequence/

pub struct Solution;

/// Reimplementation of https://leetcode.com/problems/longest-increasing-subsequence/solutions/74824/java-python-binary-search-o-nlogn-time-with-explanation/
/// Had to do this to learn!
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = vec![0; nums.len()];
		let mut size = 0;
		for num in nums {
			let (mut i, mut j) = (0, size);
			while i != j {
				let m = (i+j) / 2;
				if tails[m] < num {
					i = m+1;
				}
				else {
					j = m;
				}
			}
			tails[i] = num;
			size = size.max(i+1);
		}
		//println!("{tails:?}");
		size as i32
    }
}

#[test]
fn test() {
	fn test_lis(nums: &[i32], expected: i32) {
		let result = Solution::length_of_lis(nums.to_vec());
		println!("{nums:?} = {result} (Expected {expected})");
		assert_eq!(result, expected);
	}

	test_lis(&[4,5,6,2,3], 3);
}