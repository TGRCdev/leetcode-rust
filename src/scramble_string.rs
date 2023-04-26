pub struct Solution;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum MemoResult {
    Unscrambled(usize),
    Scrambled(usize),
    Invalid,
}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        else {
            let mut memo = HashMap::new();
            return Self::is_scramble_recurse(s1.as_str(), s2.as_str(), &mut memo);
        }
    }

    fn count_chars(split: &str) -> HashMap<char, u32> {
        let mut charcount = HashMap::new();
        for schar in split.chars() {
            charcount.entry(schar)
                .and_modify(|num| *num += 1)
                .or_insert(1);
        }
        return charcount;
    }

    fn is_scramble_recurse<'a>(s1: &'a str, s2: &'a str, memo: &mut HashMap<(&'a str, &'a str), MemoResult>) -> bool {
        if s1 == s2 {
            return true;
        }
        else if let Some(memo_result) = memo.get(&(s1, s2)) {
            //println!("({s1}) -> ({s2}) Memo'd: {memo_result:?}");
            match memo_result {
                MemoResult::Invalid => return false,
                MemoResult::Unscrambled(index) => {
                    let s1_split = s1.split_at(*index);
                    let s2_split = s2.split_at(*index);
                    return Self::is_scramble_recurse(s1_split.0, s2_split.0, memo) &&
                        Self::is_scramble_recurse(s1_split.1, s2_split.1, memo);
                },
                MemoResult::Scrambled(index) => {
                    let s1_split = s1.split_at(*index);
                    let s2_split = s2.split_at(s2.len() - *index);
                    return Self::is_scramble_recurse(s1_split.0, s2_split.1, memo) &&
                        Self::is_scramble_recurse(s1_split.1, s2_split.0, memo);
                }
            }
        }
        // Find the index of the split
        for i in 1..s1.len() {
            let s1_split = s1.split_at(i);
            let s2_split = s2.split_at(i);
            let s2_split_shuffled = s2.split_at(s2.len() - i);
            let s1_charcounts = (
                Self::count_chars(s1_split.0),
                Self::count_chars(s1_split.1),
            );
            let s2_charcounts = (
                Self::count_chars(s2_split.0),
                Self::count_chars(s2_split.1),
            );
            let s2_charcounts_shuffled = (
                Self::count_chars(s2_split_shuffled.0),
                Self::count_chars(s2_split_shuffled.1),
            );

            if s1_charcounts.0 == s2_charcounts.0 &&
                s1_charcounts.1 == s2_charcounts.1
            {
                // This is the correct split index
                //println!("String might have been split at {i} into {} and {}", s1_split.0, s1_split.1);
                memo.insert((s1, s2), MemoResult::Unscrambled(i));
                if Self::is_scramble_recurse(s1_split.0, s2_split.0, memo) &&
                    Self::is_scramble_recurse(s1_split.1, s2_split.1, memo)
                {
                    return true;
                }
            }
            if s1_charcounts.0 == s2_charcounts_shuffled.1 &&
                s1_charcounts.1 == s2_charcounts_shuffled.0 {
                //println!("String might have been split at {i} and shuffled into {} and {}", s1_split.1, s1_split.0);
                // This is the correct split index, and it was shuffled
                memo.insert((s1, s2), MemoResult::Scrambled(i));
                if Self::is_scramble_recurse(s1_split.0, s2_split_shuffled.1, memo) &&
                    Self::is_scramble_recurse(s1_split.1, s2_split_shuffled.0, memo) {
                        return true;
                    }
            }
        }

        memo.insert((s1, s2), MemoResult::Invalid);
        return false;
    }
}

#[test]
fn test() {
    fn scramble_test(s1: &str, s2: &str, expected: bool) {
        use std::time::Instant;
        let start = Instant::now();
        let result = Solution::is_scramble(s1.to_string(), s2.to_string());
        let duration = Instant::now() - start;
        println!("(\"{s1}\", \"{s2}\" = {result} (Took {} seconds)", duration.as_secs_f64());
        assert_eq!(result, expected);
    }
    scramble_test("great", "rgeat", true);
    scramble_test("abcdbdacbdac", "bdacabcdbdac", true);
    scramble_test("abb", "bba", true);
    scramble_test("eebaacbcbcadaaedceaaacadccd", "eadcaacabaddaceacbceaabeccd", false);
    scramble_test("abcde", "caebd", false);
    scramble_test("aacrbfya", "ycbarfaa", false);
}