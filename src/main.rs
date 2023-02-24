use std::thread;
use std::time;

mod part1;
mod part2;

use part1::visit_labyrinth;
use part2::sign_strat;

fn main() {
    println!("Starting part 1 (labyrinth)...");
    visit_labyrinth(10);

    thread::sleep(time::Duration::from_secs(1));
    println!("\nStarting part 2 (vase w/ signs)...");
    sign_strat(8);
}
