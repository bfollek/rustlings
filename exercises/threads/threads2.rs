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
    for _ in 0..10 {
        let status = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status = status.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
    // interesting in the output? Do you have to 'join' on all the handles?

    // The output is `jobs completed 10` for all the threads, as if they all arrive here
    // at the same time. So I don't have to join on all the handles. I don't have to join
    // on _any_handle. Is the shared Arc mutex somehow responsible for this? It must be,
    // but why? Because the Arc clones exist until all the threads are finished, and the
    // Arc ref count is therefor > 1? I dunno.

    /*
     * Use `remove` to get ownership of the handle.
     * https://stackoverflow.com/questions/68966949/unable-to-join-threads-from-joinhandles-stored-in-a-vector-rust
     */
    //let h = handles.remove(0);
    //h.join().unwrap();
    let status = status.lock().unwrap();
    println!("jobs completed {}", status.jobs_completed);
}
