/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-12 22:40:21
 * @FilePath: /rustlings/exercises/macros/macros1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
