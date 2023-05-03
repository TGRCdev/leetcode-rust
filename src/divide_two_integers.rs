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
    ) -> i32 {
        let mut span = 1;
        let mut span_divisor = divisor;
        let mut result = 0;
        while check_func(dividend, divisor) {
            if !check_func(dividend, span_divisor) {
                span = 1;
                span_divisor = divisor;
            }
            result += span;
            op_func(&mut dividend, span_divisor);

            span += span;
            span_divisor += span_divisor;
        }
        return result;
    }

    let result = match (dividend_pos, divisor_pos) {
        (true, true) => {
            perform_division(
                dividend,
                divisor,
                |dividend, divisor| dividend >= divisor,
                |dividend, divisor| *dividend -= divisor,
            )
        },
        (false,false) => {
            perform_division(
                dividend,
                divisor,
                |dividend, divisor| dividend <= divisor,
                |dividend, divisor| *dividend -= divisor,
            )
        },
        (true,false) => {
            perform_division(
                dividend,
                divisor,
                |dividend, divisor| dividend + divisor >= 0,
                |dividend, divisor| *dividend += divisor,
            )
        },
        (false,true) => {
            perform_division(
                dividend,
                divisor,
                |dividend, divisor| dividend + divisor <= 0,
                |dividend, divisor| *dividend += divisor,
            )
        }
    };

    if result_negative {
        return -result;
    }
    else {
        return result;
    }
}
}