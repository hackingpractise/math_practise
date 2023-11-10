use super::{SumReults, BUFFER};
use rand::{self, distributions::uniform::Uniform, prelude::Distribution, thread_rng};
use std::io;
use std::time::Instant;

pub(crate) fn cube(iterations: i32) -> io::Result<SumReults> {
    println!("Welcome to the cube game, try to find the cube root");
    let stdin = io::stdin();
    let range = Uniform::new(2, 101_u32);
    let mut rng = thread_rng();
    let mut score = 0_i32;
    let mut failed = vec![];
    let now = Instant::now();
    for n in 1..=iterations {
        let num = range.sample(&mut rng);
        println!("Find cube root of:{}", num.pow(3));
        let mut buffer = String::with_capacity(BUFFER);
        let _ = stdin.read_line(&mut buffer);
        let input: u32 = buffer.trim().parse().unwrap_or_default();
        if num == input {
            score += 1;
        } else {
            failed.push(n);
        }
    }
    let time_elapsed = now.elapsed();
    if failed.is_empty() {
        println!("You are incredible, Hurrah!!");
        println!(
            "You took {} seconds",
            time_elapsed.as_millis() as f64 / 1000_f64
        );
    } else {
        println!("Thanks for trying, you scored {score}");
        println!("You failed the following problems: {failed:?}");
    }
    Ok(SumReults {
        time_taken: time_elapsed.as_nanos(),
        score,
    })
}
