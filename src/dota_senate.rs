//! https://leetcode.com/problems/dota2-senate/

pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Senator {
	Radiant,
	Dire,
}

use core::convert::TryFrom;

impl TryFrom<char> for Senator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
			'D' => Ok(Senator::Dire),
			'R' => Ok(Senator::Radiant),
			_ => Err(()),
		}
    }
}

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senate_vec: Vec<Senator> = Vec::with_capacity(senate.len());
		let mut remaining_r: usize = 0;
		let mut remaining_d: usize = 0;

		senate.chars().for_each(|c| {
			let senator = c.try_into().unwrap();
			match senator {
				Senator::Dire => remaining_d += 1,
				Senator::Radiant => remaining_r += 1,
			}
			senate_vec.push(senator);
		});

		let mut banned_r = 0;
		let mut banned_d = 0;
		loop {
			let mut i = 0;
			while i < senate_vec.len() {
				match senate_vec[i] {
					Senator::Radiant => {
						if banned_r > 0 {
							banned_r -= 1;
							senate_vec.remove(i);
						}
						else if remaining_d == 0 {
							return "Radiant".to_string();
						}
						else {
							banned_d += 1;
							remaining_d -= 1;
							i += 1;
						}
					},
					Senator::Dire => {
						if banned_d > 0 {
							banned_d -= 1;
							senate_vec.remove(i);
						}
						else if remaining_r == 0 {
							return "Dire".to_string()
						}
						else {
							banned_r += 1;
							remaining_r -= 1;
							i += 1;
						}
					},
				}
			}
		}
    }
}

#[test]
fn test() {
	pub fn test_predict(senate: &str, expected: &str) {
		let result = Solution::predict_party_victory(senate.to_string());
		println!("{senate} = {result} (Expected {expected})");
		assert_eq!(expected, result.as_str());
	}

	test_predict("RD", "Radiant");
	test_predict("RDD", "Dire");
}