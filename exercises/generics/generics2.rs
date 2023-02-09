/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-08 23:46:13
 * @FilePath: /rustlings/exercises/generics/generics2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

struct Wrapper<T> {
    value: T,
}

// 泛型 为任何类型实现 struct
// Doc https://rustwiki.org/zh-CN/book/ch10-01-syntax.html#%E6%B3%9B%E5%9E%8B%E6%95%B0%E6%8D%AE%E7%B1%BB%E5%9E%8B
impl<F> Wrapper<F> {
    pub fn new(value: F) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
