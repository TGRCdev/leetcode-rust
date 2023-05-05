//! https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/

pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        if salary.len() <= 2 {
            return 0.0;
        }
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut sum = 0;
        salary.iter().for_each(|&salary| {
            min = min.min(salary);
            max = max.max(salary);
            sum += salary;
        });

        return (sum - (min + max)) as f64 / (salary.len()-2) as f64;
    }
}