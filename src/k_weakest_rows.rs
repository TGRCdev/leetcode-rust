//! https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix

pub struct Solution;

#[derive(Debug,PartialEq,Eq)]
pub struct Soldiers {
    row: i32,
    count: i32,
}

impl PartialOrd for Soldiers {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let r = self.count.cmp(&other.count);
        if r.is_eq() {
            return Some(self.row.cmp(&other.row));
        }
        else {
            return Some(r);
        }
    }
}

impl Ord for Soldiers {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        use std::{
            cmp::Reverse,
            collections::BinaryHeap,
        };

        let mut soldiers: BinaryHeap<_> = mat.into_iter()
            .enumerate()
            .map(|(i, row)| {
                Reverse(Soldiers{
                    row: i as i32,
                    count: row.into_iter().sum()
                })
            }).collect();
        
        let mut result = Vec::with_capacity(k);
        
        for _ in 0..k {
            result.push(soldiers.pop().unwrap().0.row);
        }

        result
    }
}