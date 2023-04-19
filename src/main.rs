use std::time::{Duration, Instant};

fn main() {
    cycle();
    cycleContinue();
    cycleThree();
    cycleLoop();
}

fn cycle() {
    for _ in 0..10 {
        println!("Hello, world!");
    }
}

fn cycleContinue() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("Hello, world!");
    }
}

fn cycleThree() {
    let mut count = 0;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
}

fn cycleLoop() {
    let mut count = 0;
    loop {
        count += 1;
        println!("{}", count);
        if count == 10000 {
            break;
        }
    }
}