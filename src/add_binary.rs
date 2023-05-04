//! https://leetcode.com/problems/add-binary/

pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
		let mut a = a.into_bytes();
		let mut b = b.into_bytes();

		if a.len() < b.len() {
			std::mem::swap(&mut a, &mut b);
		}

		const CHAR_0: u8 = 48;
		const CHAR_1: u8 = 49;

		'outer: for (i, byte) in b.into_iter().rev().enumerate() {
			match byte {
				CHAR_0 => continue,
				CHAR_1 => {
					// '1'
					let a_len = a.len();
					if a_len > i {
						for a_byte in a[..a_len-i].iter_mut().rev() {
							match *a_byte {
								CHAR_0 => {
									*a_byte = CHAR_1;
									continue 'outer;
								},
								CHAR_1 => {
									*a_byte = CHAR_0;
								},
								_ => panic!("Unknown character in input"),
							}
						}
					}
					// Push new digit
					a.insert(0, CHAR_1);
				}
				_ => panic!("Unknown character in input"),
			}
		}

		return unsafe { String::from_utf8_unchecked(a) };
    }
}

#[test]
fn test() {
	fn add_test(a: &str, b: &str, expected: &str) {
		let result = Solution::add_binary(a.to_string(), b.to_string());
		println!("{a} + {b} = {result} (Expected {expected})");
		assert_eq!(result.as_str(), expected);
	}

	add_test("11", "1", "100");
	add_test("1010", "1011", "10101");
	add_test("1", "111", "1000");
	add_test("11011001110011", "1011", "11011001111110");
}