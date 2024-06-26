use std::time::Duration;

use timer_state_maching::{new_executor_and_spawner, TimerFuture};

pub mod timer_state_maching;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy1!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done1!");
    });
    spawner.spawn(async {
        println!("howdy2!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done2!");
    });
    spawner.spawn(async {
        println!("howdy3!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(3, 0)).await;
        println!("done3!");
    });
    spawner.spawn(async {
        println!("howdy4!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(4, 0)).await;
        println!("done4!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}