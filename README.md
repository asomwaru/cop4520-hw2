# Homework Assignment 2

## Part 1

I implemented the first problem very similar to how I implemented part 2. Having a room that can be locked and unlocked by threads. Same thing for information regarding the cupcake.

## Part 2

I chose the second implementation for part 2 of the project. Both strategy 1 and 2 are very similar but the reason I chose 2 over 1 is because the threads can try to lock the room if possible or just do something else. The queue strategy means that idle threads won't do anything or the room could be empty while the next queued task is getting ready to enter.

## Building/Running

You can build/run the project in two ways:

### First:

```
cargo run --release -- <YOUR_ARGS_HERE>
```

### Second:

```
cargo build --release
./target/release/hw1 <YOUR_ARGS_HERE>
```
