/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-11 00:59:58
 * @FilePath: /rustlings/exercises/standard_library_types/iterators3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// iterators3.rs
// This is a bigger exercise than most of the others! You can do it!
// Here is your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a hint.

use std::iter::Map;
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32, // 被除数
    divisor: i32,  // 除数
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    // todo!();
    match b {
        0 => return Err(DivisionError::DivideByZero),
        _ => {
            let remaining = a % b;
            let quotient = a / b;

            if remaining == 0 {
                Ok(quotient)
            } else {
                Err(DivisionError::NotDivisible(NotDivisibleError {
                    dividend: a,
                    divisor: b,
                }))
            }
        }
    }
}
// helper function

// type alias
type DivResultVec_I32 = Result<Vec<i32>, DivisionError>;
type DivResult = Result<i32, DivisionError>;
fn result_iter(numbers: Vec<i32>) -> impl Iterator<Item = DivResult> {
    numbers.into_iter().map(|n| divide(n, 27))
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> DivResultVec_I32 {
    let numbers = vec![27, 297, 38502, 81];
    // let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // let result = division_results.collect();
    // result
    result_iter(numbers).collect()
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<DivResult> {
    let numbers = vec![27, 297, 38502, 81];
    // let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // let result = division_results.collect();
    // result

    result_iter(numbers).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    // 整除
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
