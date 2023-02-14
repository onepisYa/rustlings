/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-12 23:51:39
 * @FilePath: /rustlings/exercises/macros/macros4.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($($val:expr),*) => {
        println!("Look at this other macro: {}", $($val)*);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
