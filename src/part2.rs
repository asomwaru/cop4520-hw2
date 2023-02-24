use rand::Rng;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time;

#[allow(dead_code)]
pub fn sign_strat(thread_count: i32) {
    let sign = Arc::new(Mutex::new(true)); // true - room is free, false - room is busy
    let mut handler: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..thread_count {
        let sign = Arc::clone(&sign);
        let cur_thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let sleep_time = 1 + rng.gen::<u64>() % 5;
                let dur = time::Duration::from_millis(sleep_time);

                let mut room = sign.lock().unwrap();
                if *room {
                    *room = false;
                    println!("Thread {i} is in the room");

                    println!("Thread {i} is sleeping for {sleep_time} secs");
                    thread::sleep(dur);

                    *room = true;
                    break;
                }
            }

            println!("Thread {i} opening the lock");
        });

        handler.push(cur_thread);
    }

    for thread in handler {
        thread.join().unwrap();
    }
}
