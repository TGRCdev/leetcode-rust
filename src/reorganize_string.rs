//! https://leetcode.com/problems/reorganize-string/

pub struct Solution;

#[derive(Debug,Clone,Copy,Ord,Eq)]
pub struct CharCount(pub u8, pub u16);

impl PartialOrd for CharCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl PartialEq for CharCount {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl From<(u8,u16)> for CharCount {
    fn from(value: (u8,u16)) -> Self {
        Self(value.0,value.1)
    }
}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        use std::collections::{BinaryHeap, HashMap};
        #[allow(unused_imports)]
        use std::iter::FromIterator;
        let len = s.len();

        let mut charmap = HashMap::new();
        for nextchar in s.into_bytes() {
            charmap.entry(nextchar)
                .and_modify(|count| *count += 1)
                .or_insert(1u16);
        }

        let mut chars: BinaryHeap<CharCount> = BinaryHeap::from_iter(
            charmap.into_iter()
            .map(|charcount| charcount.into())
        );

        if chars.peek().unwrap().1 as usize > ((len+1)/2) {
            return String::default();
        }

        let mut result: Vec<u8> = Vec::with_capacity(len);

        let mut last_char = None;

        while let Some(mut charcount) = chars.pop() {
            if Some(charcount.0) == last_char {
                // Grab the next char instead
                let mut nextchar = chars.pop().unwrap();
                result.push(nextchar.0);
                last_char = Some(nextchar.0);
                nextchar.1 -= 1;
                if nextchar.1 > 0 {
                    chars.push(nextchar);
                }
                chars.push(charcount);
            }
            else {
                result.push(charcount.0);
                last_char = Some(charcount.0);
                charcount.1 -= 1;
                if charcount.1 > 0 {
                    chars.push(charcount);
                }
            }
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[test]
fn test() {
    let mut _test = 0;
    let mut test_case = |s: &str| {
        print!("{s} = ");
        println!("{}", Solution::reorganize_string(s.to_string()));
        _test += 1;
    };

    test_case("aabbcc");
    test_case("ababaca");
}