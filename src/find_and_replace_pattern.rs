//! https://leetcode.com/problems/find-and-replace-pattern/

pub struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(mut words: Vec<String>, pattern: String) -> Vec<String> {

		fn ascii_byte_to_index(byte: u8) -> usize {
			match byte {
				97..=122 => (byte - 97) as usize,
				_ => panic!("Not lower-case ASCII"),
			}
		}

		let pattern = {
			let mut pattern_map = [None; 26];
			let mut i = 0;
			let mut pattern_vec = Vec::with_capacity(pattern.len());
			pattern.as_bytes().into_iter().for_each(|&c| {
				let c_idx = ascii_byte_to_index(c);
				let idx = pattern_map[c_idx].unwrap_or_else(|| {
					let idx = i;
					i += 1;
					pattern_map[c_idx] = Some(idx);
					idx
				});
				pattern_vec.push(idx);
			});
			pattern_vec
		};

		fn pattern_compare(pattern: &Vec<usize>, input: &str) -> bool {
			let mut pattern_map = [None; 26];
			let mut i = 0;
			for (&pattern_idx, &input_byte) in pattern.iter().zip(input.as_bytes()) {
				let c_idx = ascii_byte_to_index(input_byte);
				let idx = pattern_map[c_idx].unwrap_or_else(|| {
					let idx = i;
					i += 1;
					pattern_map[c_idx] = Some(idx);
					idx
				});
				if pattern_idx != idx {
					return false;
				}
			}

			return true;
		}

		let mut i = 0;
		while i < words.len() {
			if pattern_compare(&pattern, words[i].as_str()) {
				i += 1;
			}
			else {
				words.swap_remove(i);
			}
		}

		words
    }
}

#[test]
fn test() {
	fn test_pattern(words: &[&str], pattern: &str, expected: &[&str]) {
		let words_vec = words.into_iter().map(|s| s.to_string()).collect();
		let mut result = Solution::find_and_replace_pattern(words_vec, pattern.to_string());
		let mut expected_vec: Vec<String> = expected.into_iter().map(|s| s.to_string()).collect();
		expected_vec.sort_unstable();
		result.sort_unstable();
		println!("{words:?} & {pattern} = {result:?} (Expected {expected:?})");
		assert_eq!(result, expected_vec);
	}

	test_pattern(&["abc","deq","mee","aqq","dkd","ccc"], "abb", &["mee","aqq"]);
}