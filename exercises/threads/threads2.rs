// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for i in 0..10 {
        let status = Arc::clone(&status);
        let handle = thread::spawn(move || {
            // Why this sleep? Does it randomize things, because when they'll
            // wake up is unpredictable? Or is it pedagogical, to give the
            // main thread a chance to finish ahead of a thread sometimes?
            thread::sleep(Duration::from_millis(250));
            // This might print out of actual order.
            println!("thread {} is waiting to lock.", i);
            let mut status = status.lock().unwrap();
            // This should print in true order, because the thread has the mutex.
            println!("thread {} has the lock.", i);
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    /*
    With this code, each thread prints `jobs completed 10`. That makes sense. They've all finished,
    and they're pointing to shared data for `status`, so they all see, and print, the same value.
    In rust terms, are they all multiple owners of the shared data? I think so.

    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        let status = status.lock().unwrap();
        println!("jobs completed {}", status.jobs_completed);
    }
    */

    /*
     * It looks like I can join on just 1 thread, and everything still works right.
     * I don't understand why joining one seems to be the same as joining all 10.
     * Either way, main doesn't get the mutex till all the threads have run.
     *
     * Actually, that falls apart when I add prints to the threads. Maybe that slows
     * them enough for main to sneak in. It's definitely happening sometimes.
     *
     * So the best answer seems to be: join on all the handles, but don't the threads.
     */

    for handle in handles {
        handle.join().unwrap();
    }

    /*
     * Use `remove` to get ownership of the handle.
     * https://stackoverflow.com/questions/68966949/unable-to-join-threads-from-joinhandles-stored-in-a-vector-rust
     */
    // let h = handles.remove(0);
    // h.join().unwrap();

    println!("main thread is waiting to lock.");
    let status = status.lock().unwrap();
    println!("main thread has the lock.");
    println!("jobs completed {}", status.jobs_completed);
}
