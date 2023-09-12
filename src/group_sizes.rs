//! https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to

pub struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut groups = vec![vec![]; group_sizes.len()];
        let mut result = vec![];
        for (id, &size) in group_sizes.iter().enumerate() {
            let index = (size as usize)-1;
            groups[index].push(id as i32);
            if groups[index].len() == size as usize {
                let mut filled_group = vec![];
                std::mem::swap(&mut groups[index], &mut filled_group);
                result.push(filled_group);
            }
        }

        result
    }
}