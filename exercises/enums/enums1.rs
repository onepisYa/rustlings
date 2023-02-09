/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-06 15:02:13
 * @FilePath: /rustlings/exercises/enums/enums1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // DONE: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
