//! https://leetcode.com/problems/minimum-string-length-after-removing-substrings/

pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        const AB: (u8, u8) = (65, 66);
        const CD: (u8, u8) = (67, 68);
        for char in s.into_bytes().into_iter() {
            if stack.is_empty() {
                stack.push(char);
                continue;
            }

            let last_char = *stack.last().unwrap();
            if matches!((last_char, char), AB | CD) {
                stack.pop();
            }
            else {
                stack.push(char);
            }
        }
        stack.len() as i32
    }
}

#[test]
fn test() {
    fn test_min_length(string: &str, expected: i32) {
        let result = Solution::min_length(string.to_string());
        println!("{string} = {result} (Expected {expected})");
        assert_eq!(result, expected);
    }

    test_min_length("ABFCACDB", 2);
    test_min_length("BCBADBBABCBABCBDABDBABCBDBACBADBCBADBACBADBCBADBCABDBCBDBDBABCBADBCDABACBBDBCBACDBDBCABDBCBAD", 67);
    test_min_length(
        ["A".repeat(400), "B".repeat(400), "C".repeat(400), "D".repeat(400)].join("").as_str(),
        0
    );
}