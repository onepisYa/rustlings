/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-09 23:12:17
 * @FilePath: /rustlings/exercises/tests/tests1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a hint.

// 此题目考验 assert! 的用法， assert! 需要一个 bool 值。
// true 则成功， false 则失败。

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);
    }
}
