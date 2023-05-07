//! https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/

pub struct Solution;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut tails = vec![0;obstacles.len()];
		let mut size = 0;

		let mut course_lengths = vec![];

		for height in obstacles {
			let (mut start, mut end) = (0, size);
			while start != end {
				let mid = (start+end)/2;
				if tails[mid] <= height {
					start = mid+1;
				}
				else {
					end = mid;
				}
			}

			tails[start] = height;
			course_lengths.push((start+1) as i32);
			size = size.max(start+1);
		}

		course_lengths
    }
}

#[test]
fn random_test() {
	use rand::prelude::*;
	const TEST_COUNT: usize = 10; // 100
	const MAX_ELEMENTS: usize = 10; //100000
	const MAX_HEIGHT: u32 = 30; // 10000000
	for i in 0..TEST_COUNT {
		let test_data: Vec<i32> = (0..(random::<usize>() % MAX_ELEMENTS)).map(|_| (random::<u32>() % MAX_HEIGHT) as i32).collect();
		let result = Solution::longest_obstacle_course_at_each_position(test_data.clone());
		println!("Test {i}: {test_data:?} = {result:?}");
	}
}

#[test]
fn test() {
	fn test_obstacles(obstacles: &[i32], expected: &[i32]) {
		let result = Solution::longest_obstacle_course_at_each_position(obstacles.to_vec());
		println!("{obstacles:?} = {result:?} (Expected {expected:?})");
		assert_eq!(result, expected);
	}

	test_obstacles(&[18, 6, 28, 14, 28, 23, 14], &[1,1,2,2,3,3,3]);
	test_obstacles(&[2,2,1], &[1,2,1]);
}