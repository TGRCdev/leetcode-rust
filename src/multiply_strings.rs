pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
		if num1 == "0" || num2 == "0" {
			return "0".to_string();
		}
		
		fn ctoi(c: char) -> u8 {
			c as u8 - 48
		}

		fn itoc(i: u8) -> char {
			(i + 48) as char
		}

		pub fn add(a: String, b: String) -> String {
			if a == "0" {
				return b;
			}
			else if b == "0" {
				return a;
			}
			use std::cmp::Ordering;
			use std::iter::repeat;
			let iter: Box<dyn Iterator<Item = (char, char)>> = {
				match a.len().cmp(&b.len()) {
					Ordering::Less => {
						Box::new(a.chars().rev().chain(repeat('0')).zip(b.chars().rev()))
					},
					Ordering::Equal => {
						Box::new(a.chars().rev().zip(b.chars().rev()))
					},
					Ordering::Greater => {
						Box::new(a.chars().rev().zip(b.chars().rev().chain(repeat('0'))))
					}
				}
			};
			let mut carry = 0;
			let mut result = Vec::with_capacity(a.len()+1);
			for (a, b) in iter.map(|(a,b)| (ctoi(a), ctoi(b))) {
				let sum = a + b;
				let mut new_carry = sum / 10;
				let mut digit = (sum % 10) + carry;
				if digit > 9 {
					new_carry += 1;
					digit %= 10;
				}
				carry = new_carry;
				result.push(itoc(digit));
			}
			if carry > 0 {
				result.push(itoc(carry));
			}

			result.into_iter().rev().collect()
		}

		let mut result = "0".to_string();
		for (zeros, multiplier) in num2.chars().rev().enumerate().map(|(i, c)| (i, ctoi(c))) {
			let mut carry = 0;
			let mut total_product = Vec::new();
			for multiplicand in num1.chars().rev().map(|c| ctoi(c)) {
				let product = (multiplicand * multiplier) + carry;
				let new_carry = product / 10;
				let digit = product % 10;
				total_product.push(itoc(digit));
				carry = new_carry;
			}
			if carry > 0 {
				total_product.push(itoc(carry))
			}
			let total_product: String = total_product.into_iter().rev().chain(std::iter::repeat('0').take(zeros)).collect();
			result = add(result, total_product);
		}

		result
    }
}

#[test]
fn test() {
	fn multiply_test(multiplicand: &str, multiplier: &str, expected: &str) {
		let result = Solution::multiply(multiplicand.to_string(), multiplier.to_string());
		println!("{multiplicand} x {multiplier} = {result} (Expected {expected})");
		assert_eq!(result, expected);
	}

	multiply_test("123", "456", "56088");
	multiply_test("190981273891273128937912371287389132", "81237981278941248917894782749124789124", "0");
}