//! https://leetcode.com/problems/lexicographically-smallest-palindrome/

pub struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(mut s: String) -> String {
        let len = s.len();
        let (left, right) = s.split_at_mut(len/2);
        let mut iter = unsafe {
            left.as_bytes_mut()
                .iter_mut()
                .zip(right
                    .as_bytes_mut()
                    .iter_mut()
                    .rev())
        };
        while let Some((left, right)) = iter.next() {
            match (*left).cmp(right) {
                std::cmp::Ordering::Less => *right = *left,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => *left = *right,
            }
        }
        s
    }
}