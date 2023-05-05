//! https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/

pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        const ASCII_A: u8 = 97;
        const ASCII_E: u8 = 101;
        const ASCII_I: u8 = 105;
        const ASCII_O: u8 = 111;
        const ASCII_U: u8 = 117;

        fn is_vowel(ascii: u8) -> bool {
            matches!(
                ascii, 
                ASCII_A | ASCII_E | ASCII_I | ASCII_O | ASCII_U,
            )
        }

        let k = k as usize;
        if k == 0 {
            return 0;
        }
        else if k == 1 {
            return s.bytes()
                .find(|&letter| is_vowel(letter))
                .is_some() as i32;
        }
        let mut vowel_count: usize = s.bytes()
            .take(k)
            .map(|byte| is_vowel(byte) as usize)
            .sum();
        let mut highest_vowel_count = vowel_count;
        let mut vowel_at_start = is_vowel(s.as_bytes()[0]);
        
        for window in s.as_bytes().windows(k).skip(1) {
            if vowel_at_start {
                vowel_count -= 1;
            }
            vowel_at_start = is_vowel(window[0]);
            if is_vowel(window[k-1]) {
                vowel_count += 1;
            }
            highest_vowel_count = highest_vowel_count.max(vowel_count);
            if highest_vowel_count == k {
                break;
            }
        }

        return highest_vowel_count as i32;
    }
}