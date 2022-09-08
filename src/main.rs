/// Monty Hall Problem numerical test (not proof)
use async_std::{
    prelude::*,
    sync::Arc,
    task::{self, block_on},
};
use clap::Parser;
use rand::{thread_rng, Rng};
use std::thread::available_parallelism;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    iterations: usize,

    #[clap(short, long)]
    num_doors: usize,
}

fn main() {
    let args = Args::parse();
    let iterations = args.iterations;
    let num_doors = args.num_doors;

    if num_doors < 3 {
        panic!("Number of doors has to be greater than or equal to 3 (doors >= 3).");
    }

    let mut tasks: Vec<task::JoinHandle<(f64, f64)>> = Vec::new();
    let parallell_count = available_parallelism().unwrap().get();
    let iterations_part = iterations / parallell_count;

    for _ in 0..parallell_count {
        tasks.push(task::spawn(async move {
            let mut rng = thread_rng();
            let mut simple = 0_usize;
            let mut smart = 0_usize;

            for _ in 0..iterations_part {
                // Set one door as "correct" (with prize)
                let correct = rng.gen_range(0..num_doors);

                // Make a random guess
                let guess = rng.gen_range(0..num_doors);

                if guess == correct {
                    // Gather data for if "simple" tactic is correct (do not switch).
                    simple += 1;
                } else {
                    // Gather data for if "smart" tactic is correct (always switch).
                    smart += 1;
                }
            }

            (
                simple as f64 / iterations_part as f64,
                smart as f64 / iterations_part as f64,
            )
        }));
    }

    let mut simple = 0_f64;
    let mut smart = 0_f64;
    let len = tasks.len() as f64;
    for t in tasks {
        let mut _simple;
        let mut _smart;

        (_simple, _smart) = block_on(t);
        simple += _simple;
        smart += _smart;
    }
    simple /= len;
    smart /= len;

    println!("Simple: {:.8}%", simple * 100_f64);
    println!("Smart: {:.8}%", smart * 100_f64);
}
