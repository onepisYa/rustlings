/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-08 22:23:51
 * @FilePath: /rustlings/exercises/error_handling/errors3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    // 这里 的 错误因为是 指定了  ParseIntError
    // 所以可以使用这个签名，如果 不知道是什么错误
    // 需要使用   Result<(), Box<dyn Error>> 这个 return 类型
    // doc https://rustwiki.org/zh-CN/book/ch09-02-recoverable-errors-with-result.html#-%E8%BF%90%E7%AE%97%E7%AC%A6%E5%8F%AF%E8%A2%AB%E7%94%A8%E4%BA%8E%E8%BF%94%E5%9B%9E-result-%E7%9A%84%E5%87%BD%E6%95%B0
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;
    // main 预期返回一个 () tuple
    // cost 这里如果错误, 将直接 return 错误
    // 下方代码不再执行。
    // Ok 则 拿到 cost 继续往下执行
    // 所以 这里的问题在于 错误的时候 return Err ，Ok 的时候 返回 ()

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    };
    // 这种方式是 返回的时候 使用 Ok() 包装一层
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
