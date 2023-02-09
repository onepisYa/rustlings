/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-06 15:16:02
 * @FilePath: /rustlings/exercises/enums/enums2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    // DONE: define the different variants used below
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
