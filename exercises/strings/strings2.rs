/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-06 15:24:04
 * @FilePath: /rustlings/exercises/strings/strings2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.



fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
