//! https://leetcode.com/problems/find-the-difference-of-two-arrays/

pub struct Solution;

impl Solution {
    pub fn find_difference(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        nums1.dedup();
        nums2.dedup();

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                nums1.remove(i);
                nums2.remove(j);
            }
            else {
                if nums1[i] > nums2[j] {
                    j += 1;
                }
                else {
                    i += 1;
                }
            }
        }

        return vec![nums1, nums2];
    }
}

#[test]
fn test() {
    fn test_arrays(nums1: &[i32], nums2: &[i32], expected: [&[i32]; 2]) {
        let result = Solution::find_difference(nums1.to_vec(), nums2.to_vec());
        println!("{nums1:?} ! {nums2:?} = {result:?} (Expected {expected:?})");
    }

    test_arrays(&[1,2,3], &[4,5,6], [&[1,2,3], &[4,5,6]]);
    test_arrays(&[1,2,3], &[2,3,4], [&[1], &[4]]);
    test_arrays(&[1,2,3,3], &[1,1,2,2], [&[3], &[]]);
}