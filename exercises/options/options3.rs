/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-07 22:45:10
 * @FilePath: /rustlings/exercises/options/options3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // 写法 1
    // 原本这里样写也是没有问题的，但是因为 Potion 没有实现
    // Copy trait，导致 所有权移动到了 match 语句中
    // match 语句结束后， p 就被释放掉了，所以 会出现编译出错。
    // 因为 y 也一起被释放掉了。
    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    // 写法 2
    // match &y {
    //     Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
    //     _ => println!("no match"),
    // }
    y; // TODO: Fix without deleting this line.
}
