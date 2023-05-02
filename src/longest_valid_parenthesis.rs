pub struct Solution;

#[derive(Debug, Clone, Copy)]
enum Layer {
    Inside(usize),
    Outside(usize),
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<Layer> = Vec::new();
        let mut longest: usize = 0;
        for (i, nextchar) in s.chars().enumerate() {
            match nextchar {
                '(' => {
                    match stack.last() {
                        Some(Layer::Inside(_)) | None => stack.push(Layer::Inside(i)),
                        Some(Layer::Outside(start)) => {
                            *stack.last_mut().unwrap() = Layer::Inside(*start);
                        }
                    }
                },
                ')' => {
                    match stack.last().cloned() {
                        Some(Layer::Outside(_)) => {
                            stack.pop();
                            if let Some(layer) = stack.last_mut() {
                                let (Layer::Inside(start) | Layer::Outside(start)) = layer;
                                let start = *start;
                                *layer = Layer::Outside(start);
                                longest = longest.max((i+1) - start);
                            }
                        },
                        Some(Layer::Inside(start)) => {
                            *stack.last_mut().unwrap() = Layer::Outside(start);
                            longest = longest.max((i+1) - start);
                        },
                        None => (),
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