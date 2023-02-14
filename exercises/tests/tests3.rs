/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-10 21:46:15
 * @FilePath: /rustlings/exercises/tests/tests3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(true);
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}
