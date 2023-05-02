pub struct Solution;

impl Solution {
    pub fn is_match(input: String, pattern: String) -> bool {
        let mut memo = vec![vec![None; pattern.len()+1]; input.len()+1];
        let input: Vec<char> = input.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();
        Self::dp(&input, &pattern, 0, 0, &mut memo)
    }
    
    fn dp(input: &[char], pattern: &[char], i_index: usize, p_index: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if let Some(result) = memo[i_index][p_index] {
            return result;
        }
        
        if p_index == pattern.len()
        {
            // If we've reached both the end of the input and
            // the pattern, then the result is true.
            let result = i_index == input.len();
            memo[i_index][p_index] = result.into();
            return result;
        }
        else
        {
            let ans;
            // Check if this pattern matches this char
            let first_match = (i_index < input.len()) && (pattern[p_index] == input[i_index] || pattern[p_index] == '.');

            // If we aren't at the end of the pattern, and the next pattern char
            // matches zero or more of this char
            if p_index + 1 < pattern.len() && pattern[p_index + 1] == '*'
            {
                ans = Self::dp(input, pattern, i_index, p_index+2, memo) // Match zero
                    || (
                        first_match &&
                        Self::dp(input, pattern, i_index+1, p_index, memo) // Match one or more
                    );
            }
            else
            {
                // Move forward
                ans = first_match && Self::dp(input, pattern, i_index+1, p_index+1, memo);
            }
            
            memo[i_index][p_index] = ans.into();
            return ans;
        }
    }
}