/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-10 23:22:57
 * @FilePath: /rustlings/exercises/standard_library_types/iterators1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a hint.


fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // DONE: Step 1
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    // str 类型是 DST 动态尺寸类型，无法在编译时获取 Size 
    // 所以只能用指针包起来，我们就知道指针的大小，然后在 运行时会知道 str 的长度的。
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple")); // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach")); // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None); // TODO: Step 4
}
