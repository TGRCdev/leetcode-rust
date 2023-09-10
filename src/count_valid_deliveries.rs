//! https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options

pub struct Solution;

impl Solution {
    const MOD: u64 = 10u64.pow(9) + 7;
    pub fn count_orders(n: i32) -> i32 {
        ((2u64..(n+1) as u64).fold::<u64,_>(1u64, |acc, num| {
            acc * (((2*num)-1)*num)
        }) % Self::MOD) as i32
    }
}