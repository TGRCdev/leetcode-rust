//! https://leetcode.com/problems/strong-password-checker/

pub struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
		/*
        aa|a
		aa|aa
		aa|a|aa
		aa|aa|aa
		a|aa|aa|aa
		f(n) = (n-1)/2
		*/
		const MIN_LENGTH: usize = 6;
		const MAX_LENGTH: usize = 20;

		const DIGIT: usize = 0;
		const UPPERCASE: usize = 1;
		const LOWERCASE: usize = 2;

		fn chartype(c: char) -> usize {
			match c {
				'0'..='9' => DIGIT,
				'A'..='Z' => UPPERCASE,
				'a'..='z' => LOWERCASE,
				_ => panic!("Unexpected character {c}"),
			}
		}

		fn count_splits_required(repeat_length: usize) -> usize {
			(repeat_length-1)/2
		}

		let mut char_reqs = [false; 3];
		let mut total_steps: usize = 0;

		let mut last_char = None;
		let mut repeat_length: usize = 1;
		let mut repeated_sections: Vec<usize> = Vec::with_capacity(password.len()/3);
		for pchar in password.chars() {
			char_reqs[chartype(pchar)] = true;
			if let Some(lchar) = last_char {
				if lchar == pchar {
					repeat_length += 1;
				}
				else {
					if repeat_length > 2 {
						repeated_sections.push(count_splits_required(repeat_length));
					}
					repeat_length = 1;
					last_char = Some(pchar);
				}
			}
			else {
				last_char = Some(pchar);
			}
		}
		if repeat_length > 2 {
			repeated_sections.push(count_splits_required(repeat_length));
		}

		// bbaaaaaaaaaaaaaaacccccc
		// bba|aa|aa|aa|aa|aa|aa|aacc|cc|cc
		// bbaaaaaaaacccc

		
		
		let mut removable_splits = 0;
		char_reqs.into_iter().for_each(|req| {
			if !req {
				if password.len() + total_steps < MIN_LENGTH {
					// Add new chars
					removable_splits += 1;
				}
				else {
					// Replace chars
					removable_splits += 1;
				}
				total_steps += 1;
			}
		});
		if password.len() + total_steps < MIN_LENGTH {
			let chars_required = MIN_LENGTH - (password.len() + total_steps);
			removable_splits += 2*chars_required;
			total_steps += chars_required;
		}
		else if password.len() > MAX_LENGTH {
			let over_limit = password.len() - MAX_LENGTH;
			removable_splits += (over_limit-1)*2;
			total_steps += over_limit;
		}
		while !repeated_sections.is_empty() {
			let splits = repeated_sections.last_mut().unwrap();
			if *splits <= removable_splits {
				removable_splits -= *splits;
				drop(splits);
				repeated_sections.pop();
			}
			else {
				*splits -= removable_splits;
				break;
			}
		}
		
		total_steps += repeated_sections.into_iter().sum::<usize>();

		return total_steps as i32;
    }
}

#[test]
fn test() {
	fn test_password(password: &str, expected: i32) {
		let result = Solution::strong_password_checker(password.to_string());
		println!("{password} = {result} (Expected {expected})");
		assert_eq!(result, expected);
	}

	test_password("Baaabb0", 1);
	test_password("a", 5);
	test_password("aA1", 3);
	test_password("1337C0d3", 0);
	test_password("aaa111", 2);
	test_password("ABABABABABABABABABAB1", 2);
	test_password("bbaaaaaaaaaaaaaaacccccc", 8);
	test_password("1111111111", 3);
}