/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-12 21:26:43
 * @FilePath: /rustlings/exercises/standard_library_types/cow1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// cow1.rs

// This exercise explores the Cow, or Clone-On-Write type.
// Cow is a clone-on-write smart pointer.
// It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
// The type is designed to work with general borrowed data via the Borrow trait.

// 介绍 Cow 类型
// 它可以包围并提供对借入数据的不可变访问，并在需要突变或所有权时懒惰地克隆数据。
// 该类型旨在通过 借用特征 与一般 借用数据一起使用

// Cow这个类型我连听都没有听说过，至少在 rust权威指南这本书中，没有提到过。
// 特性是提供一个 引用数据，然后在需要修改的时候 自动进行 clone。
// 这一节好像不用做题，仅仅 explores Cow 这个类型。

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

fn main() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    // 这时候没有发生 clone
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        // 获取 input 上的 引用数据
        Cow::Borrowed(i) => {
            println!("I borrowed the slice!");
            println!("{:#?}", i);
            // i.push(44);
            // Error, &mut &[i32] 上不存在 push 方法
        }
        _ => panic!("expected borrowed value"),
    }

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        // 在这里发生了 clone
        Cow::Owned(i) => {
            println!("I modified the slice and now own it!");
            i.push(33);
            println!("{:#?}", input);
        }
        _ => panic!("expected owned value"),
    }
    println!("{:#?}", input);

    // No clone occurs because `input` is already owned.
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(slice);
    match abs_all(&mut input) {
        // TODO
        // Cow::Borrowed(_) => println!("I own this slice!"),
        // _ => panic!("expected borrowed value"),
        Cow::Owned(i) => {
            println!("I own this slice!");
            i.push(55);
        }
        _ => panic!("expected borrowed value"),
    }
}
