/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-06 22:07:32
 * @FilePath: /rustlings/exercises/strings/strings3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.


fn trim_me(input: &str) -> String {
    // DONE: Remove whitespace from both ends of a string!
    // ???
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // DONE: Add " world!" to the string! There's multiple ways to do this!
    // ???
    let mut s = input.to_string();
    s.push_str(" world!");
    s

}

fn replace_me(input: &str) -> String {
    // DONE: Replace "cars" in the string with "balloons"!
    // ???
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
