/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-04 17:15:53
 * @FilePath: /rustlings/exercises/move_semantics/move_semantics6.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.
// 除了添加或者删除引用之外，不允许修改任何内容。

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
