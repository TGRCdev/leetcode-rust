//! https://leetcode.com/problems/frequency-tracker
//! Solved during Weekly Contest 344 https://leetcode.com/contest/weekly-contest-344/
#![allow(unused)]

struct FrequencyTracker {
	numbers: [u32;100000],
	frequencies: [u32;100001],
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {

    fn new() -> Self {
		let mut frequencies = [0;100001];
		frequencies[0] = 100000;
        Self {
			numbers: [0;100000],
			frequencies,
		}
    }
    
    fn add(&mut self, number: i32) {
		let num = number as usize - 1;
        let old_freq = self.numbers[num];
		self.frequencies[old_freq as usize] -= 1;
		self.frequencies[old_freq as usize + 1] += 1;
		self.numbers[num] += 1;
    }
    
    fn delete_one(&mut self, number: i32) {
        let num = number as usize - 1;
		if self.numbers[num] > 0 {
			let old_freq = self.numbers[num];
			self.frequencies[old_freq as usize] -= 1;
			self.frequencies[old_freq as usize - 1] += 1;
			self.numbers[num] -= 1;
		}
    }
    
    fn has_frequency(&self, frequency: i32) -> bool {
		let freq = frequency as usize;
        self.frequencies[freq] > 0
    }
}

/*
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */