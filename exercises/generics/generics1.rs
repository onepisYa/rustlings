/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-08 23:23:21
 * @FilePath: /rustlings/exercises/generics/generics1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

fn main() {
    // let mut shopping_list: Vec<?> = Vec::new();
    // let mut shopping_list: Vec<&str> = Vec::new();
    let mut shopping_list = Vec::new();
    // 如果不写 类型注解，那么需要再后面进行 push 或者一些操作
    // 能够让 编译器进行 infer 类型推断。
    shopping_list.push("milk");
}
