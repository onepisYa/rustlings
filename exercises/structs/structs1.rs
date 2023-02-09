/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-05 23:51:15
 * @FilePath: /rustlings/exercises/structs/structs1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.

struct ColorClassicStruct {
    // Done: Something goes here
    red: i32,
    green: i32,
    blue: i32,
}

struct ColorTupleStruct(/* Done: Something goes here */ i32, i32, i32);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // Done: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Done: Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Done: Instantiate a unit-like struct!
        // 单元结构体
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
