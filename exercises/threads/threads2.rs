/*
 * Copyright (c) 2023 by onepisYa pis1@qq.com , All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-12 22:10:38
 * @FilePath: /rustlings/exercises/threads/threads2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
#[derive(Debug)]
struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // let status = Arc::new(JobStatus { jobs_completed: 0 });
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // status_shared.jobs_completed += 1;
            let mut mutex_status_shared = status_shared
                .lock()
                .expect("status_shared acquire lock error");
            mutex_status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // let s = status.lock().unwrap();
    // 会不会死锁？看来是 死了。
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        // 不 join 可能导致的问题是，主线程运行完之后，子线程也会死掉。
        // 所以需要join 否则 一直为 0
        // println!("jobs completed {}", ???);
        let s = status.lock().unwrap();
        println!("jobs completed {:?}", s.jobs_completed);
        // 不能 在 join 之前 获取锁，否则会导致 死锁。
        // 因为 join 的时候确保了所有线程都能运行
        // 我们在 join 之前 获取了锁，join 在这个地方会一致等 线程结束，
        // 但是 线程在等 我们离开作用域，所以死锁。
    }
    let s = status.lock().unwrap();
    println!("jobs completed {:?}", s.jobs_completed)
}
