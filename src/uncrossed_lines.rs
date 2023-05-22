//! https://leetcode.com/problems/uncrossed-lines/

pub struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut lcs = vec![0; nums2.len()+1];

        for &num1 in nums1.iter() {
            let mut corner_val = 0;
            for (i, &num2) in nums2.iter().enumerate() {
                let i = i+1;
                let next_corner_val = lcs[i];
                if num1 == num2 {
                    lcs[i] = corner_val+1;
                }
                else {
                    lcs[i] = lcs[i].max(lcs[i-1]);
                }
                corner_val = next_corner_val;
            }
            //println!("{lcs:?}");
        }

        lcs[nums2.len()]
    }
}

#[test]
fn test() {
    fn test_uncrossed(nums1: &[i32], nums2: &[i32], expected: i32) {
        let result = Solution::max_uncrossed_lines(nums1.to_vec(), nums2.to_vec());
        println!("{nums1:?} LCS {nums2:?} = {result} (Expected {expected})");
        assert_eq!(result, expected);
    }

    test_uncrossed(&[1,2,4], &[1,4,2], 2);
    test_uncrossed(&[2,5,1,2,5],&[10,5,2,1,5,2], 3);
    test_uncrossed(&[3,3],&[3], 1);
}