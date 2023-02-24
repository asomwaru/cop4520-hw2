use rand::Rng;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time;

pub fn visit_labyrinth(guests_count: i32) {
    let available = Arc::new(Mutex::new(true)); // true - available is free, false - available is busy
    let cupcake = Arc::new(Mutex::new(true)); // true - cupcake is there, false - cupcake is gone
    let mut handler: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..guests_count {
        let available_clone = Arc::clone(&available);
        let cupcake_clone = Arc::clone(&cupcake);

        let cur_thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut visits = 0;

            loop {
                let mut visitable = available_clone.lock().unwrap();
                let mut cupcake = cupcake_clone.lock().unwrap();

                if *visitable {
                    visits += 1;

                    *visitable = false;
                    println!("Guest {i} is in the labyrinth for {visits} time(s).");

                    if !(*cupcake) {
                        println!("Guest {i} found no cupcake.");
                        *cupcake = 0.5 >= rng.gen::<f64>();

                        if *cupcake {
                            println!("Guest {i} requested a cupcake!");
                        }
                    }

                    if *cupcake && 0.5 >= rng.gen::<f64>() {
                        *cupcake = false;
                        println!("Guest {i} ate the cupcake!");
                    } else {
                        println!("Guest {i} didn't eat the cupcake.");
                    }

                    *visitable = true;
                    if 0.5 >= rng.gen::<f64>() {
                        println!("Guest {i} is leaving the labyrinth for good");
                        break;
                    }
                }

                // drop the lock, otherwise thread will hold it for however long and then release when completely done
                drop(visitable);
                thread::sleep(time::Duration::from_millis(1));
            }
        });

        handler.push(cur_thread);
    }

    for thread in handler {
        thread.join().unwrap();
    }
}
