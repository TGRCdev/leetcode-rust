//! https://leetcode.com/problems/substring-with-concatenation-of-all-words/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut wordmap = HashMap::with_capacity(words.len());
        let mut next_index = 0;
        for word in words.iter() {
            wordmap.entry(word.as_str())
                .and_modify(|(_, count)| *count += 1)
                .or_insert_with(|| {
                    let idx = next_index;
                    next_index += 1;
                    (idx, 1)
                });
        }
        let mut wordcount = vec![0; wordmap.len()];
        wordmap.iter().for_each(|(_, &(idx, count))| {
            wordcount[idx] = count;
        });

        let word_length = words[0].len();
        let total_substr_length = words.len() * word_length;

        let mut word_memo = vec![-1; s.len()-(word_length-1)];
        // -1 = No word
        // 0 and up = Word index

        let mut indices = Vec::new();

        for i in (0..s.len()-(word_length-1)).rev() {
            if let Some(&(word_index, _)) = wordmap.get(&s[i..i+word_length]) {
                word_memo[i] = word_index as i16;
                if s.len() - i >= total_substr_length-word_length {
                    let mut words_found = vec![0; wordmap.len()];
                    let mut word_count = 1;
                    words_found[word_index] = 1;
                    let mut j = i + word_length;
                    while word_count < words.len() && j < s.len() {
                        if word_memo[j] >= 0 {
                            let word_index = word_memo[j] as usize;
                            if words_found[word_index] < wordcount[word_index] {
                                words_found[word_index] += 1;
                                word_count += 1;
                            }
                            else {
                                // Duplicate word
                                break;
                            }
                            j += word_length;
                        }
                        else {
                            break;
                        }
                    }

                    if wordcount == words_found {
                        // Full substring
                        indices.push(i as i32);
                    }
                }
            }
        }

        for pair in wordmap.iter() {
            println!("{pair:?}");
        }
        println!("{word_memo:?}");

        return indices;
    }
}

#[test]
fn test() {
    fn substring_test(s: &str, words: &[&str], expected: &[i32]) {
        let words_vec = words.into_iter().map(|s| s.to_string()).collect();
        let result = Solution::find_substring(s.to_string(), words_vec);
        println!("{s} with {words:?} = {result:?} (Expected {expected:?})");
    }

    substring_test("barfoothefoobarman", &["foo", "bar"], &[9, 0]);
    substring_test("wordgoodgoodgoodbestword", &["word", "good", "best", "good"], &[8]);
}