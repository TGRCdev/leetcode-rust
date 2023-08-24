//! https://leetcode.com/problems/reorganize-string/

pub struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let len = s.len();
        use std::collections::HashMap;
        let mut charcount = HashMap::new();
        s.into_bytes().into_iter().for_each(|c| {
            charcount.entry(c)
                .and_modify(|n| *n += 1u16)
                .or_insert(1);
        });
        println!("{charcount:?}");
        let mut chars: Vec<(u8, u16)> = charcount.into_iter().collect();
        // Descending sort
        chars.sort_unstable_by_key(|&e| e.1);

        if chars[chars.len()-1].1 as usize > ((len+1)/2) {
            return String::default();
        }

        let mut result = Vec::with_capacity(len);
        let mut swap = false;
        let mut index = 0;

        {
            while !chars.is_empty() {
                let chars_len = chars.len();
                if swap {
                    let swap_index = chars_len-(index+2);
                    let next_char = chars[swap_index];
                    result.push(next_char.0);
                    if next_char.1 <= 1 {
                        chars.swap_remove(swap_index);
                    }
                    else {
                        chars[swap_index].1 -= 1;
                        index += 1;
                        if index == chars_len-1 {
                            index = 0;
                        }
                    }
                    swap = !swap;
                }
                else {
                    let next_char = chars[chars_len-1];
                    result.push(next_char.0);
                    if next_char.1 <= 1 {
                        chars.pop();
                        if chars.len() > 1 {
                            swap = !swap;
                        }
                    }
                    else {
                        chars[chars_len-1].1 -= 1;
                        swap = !swap;
                    }
                }
                println!("{result:?}");
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