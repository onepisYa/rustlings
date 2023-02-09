/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-09 19:49:11
 * @FilePath: /rustlings/exercises/traits/traits2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

// DONE: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
