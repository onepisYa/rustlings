/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-09 19:45:55
 * @FilePath: /rustlings/exercises/traits/traits1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar` for the type `String`.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // DONE: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
