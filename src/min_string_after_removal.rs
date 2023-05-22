//! https://leetcode.com/problems/minimum-string-length-after-removing-substrings/

pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut s = s.into_bytes();
        const AB: &[u8] = &[65, 66];
        const CD: &[u8] = &[67, 68];
        'outer: loop {
            for i in 0..s.len()-1 {
                if matches!(&s[i..=i+1], AB | CD) {
                    s[i..].rotate_left(2);
                    s.truncate(s.len()-2);
                    if s.len() == 0 { return 0; }
                    continue 'outer;
                }
            }
            break; // No more matches
        }
        s.len() as i32
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