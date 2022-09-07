/// Monty Hall Problem numerical test (not proof)
use clap::Parser;
use rand::{thread_rng, Rng};

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

    let mut rng = thread_rng();
    let mut simple = Vec::<bool>::with_capacity(iterations);
    let mut smart = Vec::<bool>::with_capacity(iterations);

    for _ in 0..iterations {
        // Set one door as "correct" (with prize)
        let correct = rng.gen_range(0..num_doors);

        // Make a random guess
        let guess = rng.gen_range(0..num_doors);

        // Gather data for if "simple" tactic is correct (do not switch).
        simple.push(guess == correct);

        // Gather data for if "smart" tactic is correct (always switch).
        smart.push(guess != correct);
    }

    let print_format = |prefix: &str, v: &Vec<bool>| {
        let fun = |acc, &x| {
            if x {
                acc + 1
            } else {
                acc
            }
        };
        println!(
            "{}: {:.8}%",
            prefix,
            (v.iter().fold(0, fun) as f64 / iterations as f64) * 100_f64
        );
    };

    print_format("Simple", &simple);
    print_format("Smart", &smart);
}
