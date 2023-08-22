//! https://leetcode.com/problems/excel-sheet-column-title/

pub struct Solution;

// I think Leetcode's version of Rust doesn't include this in the prelude.
use std::iter::FromIterator;

static CHARTABLE: [char; 26] = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M',
    'N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
];

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut buffer = ['\0','\0','\0','\0','\0','\0','\0'];
        let mut length = 0;
        let mut current_num = column_number-1;
        loop {
            buffer[7-length] = CHARTABLE[(current_num%26) as usize];
            length += 1;
            if current_num < 26 { break; }
            current_num /= 26;
            current_num -= 1;
        }
        String::from_iter(&buffer[7-length..])
    }
}