/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-08 09:21:24
 * @FilePath: /rustlings/exercises/error_handling/errors2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// errors2.rs
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy,
// and the `total_cost` function will calculate the total number of tokens.
// Since the player typed in the quantity, though, we get it as a string-- and
// they might have typed anything, not just numbers!

// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is:
// if we call the `parse` function on a string that is not a number, that
// function will return a `ParseIntError`, and in that case, we want to
// immediately return that error from our function and not try to multiply
// and add.

// There are at least two ways to implement this that are both correct-- but
// one is a lot shorter!
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a hint.


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    // 写法 1
    // here is the shorter way
    // A Shortcut for Propagating Errors: the ? Operator
    // http://doc.rust-lang.org/1.66.1/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    // let qty = item_quantity.parse::<i32>()?;
    // 写法 2
    let qty_result = item_quantity.parse::<i32>();
    let qty_number = match qty_result {
        Ok(num) => return Ok(num * cost_per_item + processing_fee),
        Err(err) => return Err(err),
    };
    // 两种写法本质上是一样的，但是 ? 是一种快捷方式，依赖于 内部实现的 from trait
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
