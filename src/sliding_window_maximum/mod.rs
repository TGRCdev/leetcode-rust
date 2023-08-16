pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BTreeMap;
        let k = k as usize;
        let mut windowmap: BTreeMap<i32, u32> = BTreeMap::new();
        // Populate the first window
        for &num in &nums[..k] {
            windowmap.entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1u32);
        }

        let mut max_nums = vec![*windowmap.last_key_value().unwrap().0];

        for window in nums.windows(k+1) {
            let cropped = &window[0];
            let last = window[k];
            if windowmap.get(cropped).copied() == Some(1) {
                windowmap.remove(cropped);
            }
            else {
                *windowmap.get_mut(&window[0]).unwrap() -= 1;
            }

            windowmap.entry(last)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            
            max_nums.push(*windowmap.last_key_value().unwrap().0);
        }

        return max_nums;
    }
}

#[test]
fn max_sliding_window_test() {
    let mut test_num = 0;
    let mut test_case = |input: &[i32], k: i32, expected: &[i32]| {
        let input = input.to_vec();
        let result = Solution::max_sliding_window(input, k);
        test_num += 1;
        println!("Test Case {test_num} - {}", if result.as_slice() == expected {"SUCCESS"} else {"FAIL"});
        //println!("{:?}", result);
    };

    test_case(include!("long_test_case.txt"), 10000, include!("long_test_case_expected.txt"));
}