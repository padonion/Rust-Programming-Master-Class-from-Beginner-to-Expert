use std::thread;
use std::time::Duration;

fn main() {
    let job_1 = thread::spawn(|| {
        println!("-- Job 1 started --");
        println!("Waiting for Job 2 to finish");
        thread::park_timeout(Duration::from_secs(2));
        println!("-- Job 1 resumed --");
        println!("-- Job 1 finished --");
    });

    let job_2 = thread::spawn(|| {
        println!("-- Job 2 started --");
        println!("-- Job 2 finished --");
    });

    job_2.join().unwrap();
    //job_1.thread().unpark();
    job_1.join().unwrap();
}
