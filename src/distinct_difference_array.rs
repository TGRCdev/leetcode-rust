//! https://leetcode.com/problems/find-the-distinct-difference-array
//! Solved during Weekly Contest 344 https://leetcode.com/contest/weekly-contest-344/

pub struct Solution;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
		fn count_distinct(nums: &[i32]) -> (usize, [usize; 51]) {
			let mut distinct_map = [0; 51];
			let mut distinct = 0;
			for &num in nums {
				if distinct_map[num as usize] == 0 {
					distinct += 1;
				}
				distinct_map[num as usize] += 1;
			}

			return (distinct, distinct_map);
		}

		let mut prefix = 1;
		let mut prefixmap = vec![0; 51];
		prefixmap[nums[0] as usize] = 1;
		let (mut suffix, mut suffixmap) = count_distinct(&nums[1..]);

		let mut diff = vec![prefix as i32 - suffix as i32];

		for num in nums.into_iter().skip(1) {
			if prefixmap[num as usize] == 0 {
				prefix += 1;
			}
			prefixmap[num as usize] += 1;
			if suffixmap[num as usize] == 1 {
				suffix -= 1;
			}
			suffixmap[num as usize] = suffixmap[num as usize].checked_sub(1).unwrap_or(0);

			diff.push(prefix as i32 - suffix as i32);
		}

		return diff;
    }
}