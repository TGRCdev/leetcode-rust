//! https://leetcode.com/problems/minimum-penalty-for-a-shop/

pub struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        const Y: u8 = 089;
        const N: u8 = 078;

        let mut lowest = 0;
        let mut lowest_sum = 0;
        
        let mut sum = 0;

        customers.into_bytes().into_iter().enumerate().for_each(|(i, byte)| {
            match byte {
                Y => sum -= 1,
                N => sum += 1,
                _ => panic!("Bad string"),
            }

            if sum < lowest_sum {
                lowest_sum = sum;
                lowest = i+1;
            }
        });

        lowest as i32
    }
}