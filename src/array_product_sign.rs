//! https://leetcode.com/problems/sign-of-the-product-of-an-array/

pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut positive = true;
        for i in &nums {
            match i {
                0 => return 0,
                1..=i32::MAX => (),
                i32::MIN..=-1 => positive = !positive,
            }
        }
        return if positive {1} else {-1};
    }
}