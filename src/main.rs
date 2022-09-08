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
        let mut simple = Vec::<bool>::with_capacity(iterations_part);
        let mut smart = Vec::<bool>::with_capacity(iterations_part);

        tasks.push(task::spawn(async move {
            let mut rng = thread_rng();

            for _ in 0..iterations_part {
                // Set one door as "correct" (with prize)
                let correct = rng.gen_range(0..num_doors);

                // Make a random guess
                let guess = rng.gen_range(0..num_doors);

                // Gather data for if "simple" tactic is correct (do not switch).
                simple.push(guess == correct);

                // Gather data for if "smart" tactic is correct (always switch).
                smart.push(guess != correct);
            }

            let fold = |v: &Vec<bool>| {
                let fun = |acc, &x| {
                    if x {
                        acc + 1
                    } else {
                        acc
                    }
                };

                (v.iter().fold(0, fun) as f64 / iterations_part as f64) * 100_f64
            };

            (fold(&simple), fold(&smart))
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

    println!("Simple: {:.8}%", simple);
    println!("Smart: {:.8}%", smart);
}
