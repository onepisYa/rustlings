/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-04 17:03:17
 * @FilePath: /rustlings/exercises/move_semantics/move_semantics5.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 100;
    let y = &mut x;
    // 不允许多个可变引用
    // 提示我们不允许添加和删除代码
    // 仅仅通过 修改顺序进行代码的修复
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
