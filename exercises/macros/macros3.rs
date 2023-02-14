/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-12 23:46:21
 * @FilePath: /rustlings/exercises/macros/macros3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// #[macro_use]
// doc http://doc.rust-lang.org/1.67.0/reference/macros-by-example.html#the-macro_use-attribute
// 方法一 添加  #[macro_use] 才能使用 模块中的宏
mod macros {
    // 方法二 在这里导出也是可以的
    #[macro_export]
    // TODO: 我很好奇 如何手动 导出 和 导入 一个宏定义，
    // 而不是 使用官方提供的宏定义
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!()
}
