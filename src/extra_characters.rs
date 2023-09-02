//! https://leetcode.com/problems/extra-characters-in-a-string/description/

pub struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
		use std::collections::HashMap;

        let mut wordmap: HashMap<u8, Vec<&str>> = HashMap::new();
		dictionary.iter().for_each(|word| {
			let firstchar = word.as_bytes()[0];
			wordmap.entry(firstchar)
				.and_modify(|list| list.push(word))
				.or_insert(vec![word]);
		});

		wordmap.values_mut().for_each(|list| list.sort_unstable_by_key(|word| usize::MAX - word.len()));

		let mut dp = vec![0; s.len()+1];
		for i in (0..s.len()).rev() {
            dp[i] = dp[i+1] + 1;
			let firstchar = s.as_bytes()[i];
			if let Some(list) = wordmap.get(&firstchar) {
				let slice = &s[i..];
				for &word in list {
					if slice.starts_with(word) {
						dp[i] = dp[i].min(dp[i+word.len()]);
					}
				}
			}
		}

        //println!("{dp:?}");

		dp[0]
    }
}