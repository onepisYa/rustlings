/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-09 20:21:54
 * @FilePath: /rustlings/exercises/traits/traits5.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a hint.

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T: OtherTrait + SomeTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
