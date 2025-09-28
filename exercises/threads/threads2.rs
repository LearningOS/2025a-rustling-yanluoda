// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// threads2.rs

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Mutex 包装 JobStatus 以实现线程安全的数据修改
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 获取互斥锁并修改共享值
            let mut status_guard = status_shared.lock().unwrap();
            status_guard.jobs_completed += 1;
            // 锁在 status_guard 离开作用域时自动释放
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
        // 打印当前的 jobs_completed 值
        let status_guard = status.lock().unwrap();
        println!("jobs completed {}", status_guard.jobs_completed);
    }
}