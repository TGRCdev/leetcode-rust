pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut longest: usize = 0;
        for (i, nextchar) in s.chars().enumerate() {
            match nextchar {
                '(' => stack.push(i),
                ')' => {
                    if stack.len() > 1 {
                        stack.pop();
                        let start = stack.last().unwrap();
                        longest = longest.max((i+1) - start);
                    }
                    else if !stack.is_empty() {
                        stack[0] = i;
                    }
                },
                _ => panic!("Unexpected character {}", nextchar),
            }
        }

        return longest as i32;
    }
}

#[test]
fn test() {
    fn try_solution(s: &str, expected: i32) {
        let result = Solution::longest_valid_parentheses(s.to_string());
        println!("{s} = {result} (Expected {expected})");
    }

    try_solution("(()", 2);
    try_solution(")()())", 4);
    try_solution("", 0);
}