/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-13 20:26:05
 * @FilePath: /rustlings/exercises/conversions/using_as.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// Type casting in Rust is done via the usage of the `as` operator.
// Please note that the `as` operator is not only used when type casting.
// It also helps with renaming imports.
//
// The goal is to make sure that the division does not fail to compile
// and returns the proper type.
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a hint.

// NOTE: as 操作符不仅仅用于 type  casting
// NOTE: 也帮助用于重命名 导入

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // total / values.len()
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
