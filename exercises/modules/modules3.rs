/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-06 22:16:12
 * @FilePath: /rustlings/exercises/modules/modules3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// modules3.rs
// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a hint.


// DONE: Complete this use statement
// use ???
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
