/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-07 22:37:26
 * @FilePath: /rustlings/exercises/options/options2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // DONE: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // DONE: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        // 写法 1
        // if let Some(Some(integer)) = optional_integers.pop() {
        //     assert_eq!(integer, range);
        //     range -= 1;
        // }
        // 写法 2
        if let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, Some(range));
            range -= 1;
        }
    }
}
