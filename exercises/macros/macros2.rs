/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-12 22:45:15
 * @FilePath: /rustlings/exercises/macros/macros2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// 宏定义 遵循 一定的顺序。
// 函数在 rust 中 可以定义在后面。
// 但是前面的 函数依然可以使用。
// 这和 动态语言不同。


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
    hello();
}

fn hello() {
    println!("Hello World!")
}
