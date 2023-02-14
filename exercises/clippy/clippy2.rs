/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-13 00:04:20
 * @FilePath: /rustlings/exercises/clippy/clippy2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option {
    //     res += x;
    // }

    // 下面的写法和上面的写法是一样的，但是 clippy 推荐
    // 使用下面的写法。
    if let Some(x) = option {
        res += x
    }
    println!("{}", res);
}
