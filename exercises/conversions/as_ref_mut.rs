/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-14 22:34:52
 * @FilePath: /rustlings/exercises/conversions/as_ref_mut.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description
 */
// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a hint.

// 引用到引用的 转换。
//  AsRef  和  AsMut

// use std::str::Bytes;

// Obtain the number of bytes (not characters) in the given argument.
// DONE: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}
// Obtain the number of characters (not bytes) in the given argument.
// DONE: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// DONE: Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T)
// where
//     &mut T: Mul<i32, Output = ()>,
{
    // DONE: Implement the function body.
    // ???
    // NOTE: 这里面 主要考的是 可变引用
    // 了解直接操作引用，可以修改 外部的值，我们无需返回任何内容。
    *arg.as_mut() = arg.as_mut().pow(2);
    // NOTE: pow 函数 计算指数的。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
