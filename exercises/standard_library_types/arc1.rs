/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-12 20:46:03
 * @FilePath: /rustlings/exercises/standard_library_types/arc1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// arc1.rs
// In this exercise, we are given a Vec of u32 called "numbers" with values ranging
// from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ]
// We would like to use this set of numbers within 8 different threads simultaneously.
// Each thread is going to get the sum of every eighth value, with an offset.
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...

// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.

// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// 原子引用计数器
// 让代码编译，并填充 shared_numbers
// 第二个TODO 不要创建 numbers 的 copies
// 所以我们仅仅使用 Arc::clone 来增加引用计数，并不需要 clone

#![forbid(unused_imports)]
// Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    // Done
    // numbers 不能直接放到线程去使用，需要在外部生成一个 shared_numbers
    // 在线程中进行 数据共享
    // 然后在 线程中使用 child_numbers 来进行计算。
    // 因为多个线程所以需要 _多所有权_ 。
    // 线程间共享数据 内部可变性 需要 Mutex 互斥锁，。
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        // DONE
        // 这里使用 Arc::clone 来增加 原子引用计数器。
        // 因为仅仅只是 读取数据并没有修改，所以我们不需要使用 Mutex 互斥锁。
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
