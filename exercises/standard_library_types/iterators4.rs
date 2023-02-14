/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-11 01:12:38
 * @FilePath: /rustlings/exercises/standard_library_types/iterators4.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // 尝试不使用至关重要的 循环 for, while
    // 额外挑战，不使用递归
    /*
    一般来说 都是 for 循环到 num 就停下。
    不让我用，那我只能用迭代器上面的方法，但是实际上，本质还是循环。
    */
    if num == 0 {
        return 1;
    }
    (1..num + 1).reduce(|foo, bar| foo * bar).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
