/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-13 20:23:13
 * @FilePath: /rustlings/exercises/clippy/clippy3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

// 我不懂这个练习想要干嘛。

use ::std::mem;
#[allow(unused_variables, unused_assignments)]
fn main() {
    // 所以这里想要我做啥？
    // 这段代码最好就是删除掉咯。
    // 下面几种写法都是有问题的，因为没有任何意义。
    let my_option: Option<()> = None;
    // if my_option.is_none() {
    if my_option.is_none() {
        // my_option.unwrap();
        println!("おはよ!")
    }

    // if let Some(_) = my_option {
    //     my_option.unwrap();
    // }

    // if my_option.is_some() {
    //     my_option.unwrap();
    // }

    let my_arr = &[-1, -2, -3 - 4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // NOTE: 这里也是没有意义的。
    // 当然如果说考验的是 如何清空一个 Vec  当我没有说过。
    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    // NOTE: 下面这句和 上面 行想要表达的是一样的，上面的是错误的写法。
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
