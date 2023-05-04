//! https://leetcode.com/problems/divide-two-integers/

pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        match divisor {
            1 => return dividend,
            -1 => {
                if dividend == i32::MIN {
                    return i32::MAX;
                }
                else {
                    return -dividend;
                }
            },
            _ => (),
        }
        if divisor == dividend {
            return 1;
        }
        let (dividend_pos, divisor_pos) = (dividend.is_positive(), divisor.is_positive());
        let result_negative = dividend_pos != divisor_pos;

        fn perform_division<
            CheckFunc: Fn(i32,i32) -> bool,
            OpFunc: Fn(&mut i32,i32) -> (),
        >(
            mut dividend: i32,
            divisor: i32,
            check_func: CheckFunc,
            op_func: OpFunc,
        ) -> usize {
            let mut divisor_powers: Vec<(usize, i32)> = Vec::with_capacity(32);
            divisor_powers.push((1, divisor));
            for i in 0..31 {
                if let Some(next_divisor) = divisor_powers[i].1.checked_add(divisor_powers[i].1) {
                    if !check_func(dividend, next_divisor) {
                        break;
                    }
                    divisor_powers.push((divisor_powers[i].0 + divisor_powers[i].0, next_divisor));
                }
                else {
                    break;
                }
            }
            let mut result = 0;
            while !divisor_powers.is_empty() {
                let divisor = divisor_powers.last().copied().unwrap();
                result += divisor.0;
                op_func(&mut dividend, divisor.1);

                while !divisor_powers.is_empty() && !check_func(dividend, divisor_powers.last().copied().unwrap().1) {
                    divisor_powers.pop();
                }
            }
            return result;
        }

        let result = match (dividend_pos, divisor_pos) {
            (true, true) => {
                if divisor > dividend {
                    return 0;
                }
                perform_division(
                    dividend,
                    divisor,
                    |dividend, divisor| dividend >= divisor,
                    |dividend, divisor| *dividend -= divisor,
                )
            },
            (false,false) => {
                if divisor < dividend {
                    return 0;
                }
                perform_division(
                    dividend,
                    divisor,
                    |dividend, divisor| dividend <= divisor,
                    |dividend, divisor| *dividend -= divisor,
                )
            },
            (true,false) => {
                if -dividend > divisor {
                    return 0;
                }
                else if -dividend == divisor {
                    return -1;
                }
                perform_division(
                    dividend,
                    divisor,
                    |dividend, divisor| dividend + divisor >= 0,
                    |dividend, divisor| *dividend += divisor,
                )
            },
            (false,true) => {
                if dividend > -divisor {
                    return 0;
                }
                else if dividend == -divisor {
                    return -1;
                }
                perform_division(
                    dividend,
                    divisor,
                    |dividend, divisor| dividend + divisor <= 0,
                    |dividend, divisor| *dividend += divisor,
                )
            }
        };

        if result_negative {
            return -(result as i32);
        }
        else {
            return result as i32;
        }
    }
}

#[test]
fn test() {
    fn test_divide(dividend: i32, divisor: i32, expected: i32) {
        let result = Solution::divide(dividend, divisor);
        println!("{dividend} / {divisor} = {result} (Expected {expected})");
        assert_eq!(result, expected);
    }

    test_divide(10, 3, 3);
    test_divide(7, -3, -2);
    test_divide(-2147483648, -1, 2147483647);
    test_divide(-2147483648, -3, 715827882);
    test_divide(1,2,0);
    test_divide(-2147483648, 2, -1073741824);
    test_divide(10, -10, -1);
}