/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-10 23:16:36
 * @FilePath: /rustlings/exercises/lifetimes/lifetimes3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
