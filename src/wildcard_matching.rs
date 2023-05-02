pub struct Solution;

impl Solution {
    pub fn is_match(input: String, pattern: String) -> bool {
        let (i_empty, p_empty) = (input.is_empty(), pattern.is_empty());
        match (i_empty, p_empty) {
            (true, true) => return true,
            (false, true) => return false,
            _ => (),
        }
        let mut memo = vec![vec![None; pattern.len()+1]; input.len()+1];
        let input: Vec<char> = input.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();
        Self::dp(&input, &pattern, 0, 0, &mut memo)
    }
    
    fn dp(input: &[char], pattern: &[char], i_index: usize, p_index: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if let Some(result) = memo[i_index][p_index] {
            return result;
        }

        let (p_end, i_end) = (p_index == pattern.len(), i_index == input.len());
        
        let ans = match (p_end, i_end) {
            (true, true) => true, // End of pattern and input
            (true, false) => pattern[p_index-1] == '*', // End of pattern, not input
            (false, true) => pattern[p_index] == '*' && Self::dp(input, pattern, i_index, p_index+1, memo), // End of input, not pattern
            (false, false) => {
                // Check if this pattern matches this char
                match pattern[p_index] {
                    '?' => {
                        Self::dp(input, pattern, i_index+1, p_index+1, memo)
                    },
                    '*' => {
                        Self::dp(input, pattern, i_index, p_index+1, memo) || // Match zero
                            Self::dp(input, pattern, i_index+1, p_index+1, memo) || // Match one
                            Self::dp(input, pattern, i_index+1, p_index, memo) // Match more
                    },
                    _ => {
                        input[i_index] == pattern[p_index] && Self::dp(input, pattern, i_index+1, p_index+1, memo)
                    },
                }
            },
        };

        memo[i_index][p_index] = Some(ans);
        ans
    }
}

#[test]
fn test() {
    fn match_test(input: &str, pattern: &str, expected: bool) {
        let result = Solution::is_match(input.to_string(), pattern.to_string());
        println!("{input} & {pattern} = {result} (Expected {expected})");
    }

    match_test("aa", "*", true);
    match_test("", "*****", true);
    match_test("a", "", false);
}