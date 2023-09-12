//! https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique

pub struct Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        use std::collections::hash_map::{HashMap, Entry};
        let mut counts: HashMap<u8, u32> = HashMap::new();
        for c in s.as_bytes() {
            counts.entry(*c)
                .and_modify(|count| *count = count.checked_add(1).unwrap())
                .or_insert(1);
        }

        let mut chars_in_counts = HashMap::new();
        for (nextchar, count) in counts {
            chars_in_counts.entry(count)
                .and_modify(|counts: &mut Vec<u8>| counts.push(nextchar))
                .or_insert_with(|| vec![nextchar]);
        }

        let mut keys: Vec<_> = chars_in_counts.keys().copied().collect();
        keys.sort_unstable();

        let mut deletions = 0;
        for key in keys {
            let mut leftover_chars = chars_in_counts[&key].len();
            while leftover_chars > 1 {
                let cur_char = chars_in_counts.get_mut(&key).and_then(|v| v.pop()).unwrap();
                if chars_in_counts.get(&key).unwrap().is_empty() {
                    chars_in_counts.remove(&key);
                }
                for i in (1..key.into()).rev() {
                    deletions += 1;
                    if let Entry::Vacant(e) = chars_in_counts.entry(i) {
                        e.insert(vec![cur_char]);
                        break;
                    }
                }
                leftover_chars -= 1;
            }
        }

        deletions
    }
}