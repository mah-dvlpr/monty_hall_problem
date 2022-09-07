/// Monty Hall Problem numerical test (not proof)

use rand::{thread_rng, Rng};
use clap::Parser;

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
        let mut doors = vec![false; num_doors];
        let correct = rng.gen_range(0..num_doors);
        doors[correct] = true;

        // Make a random guess
        let guess = rng.gen_range(0..num_doors);

        // Gather data for if "simple" tactic is correct.
        simple.push(doors[guess]);

        // Gather data for if "smart" tactic is correct.
        // Truncate all wrong options except guess and correct (in new vector). Would be done by the show host.
        if guess != correct {
            doors = vec![doors[guess], doors[correct]];
        } else {
            // "Keep a bad door"
            doors = vec![doors[guess], false];
        }
        
        // From logic above, "switching" means always picking the second door (index 1),
        // since our initial guess will have index 0.
        smart.push(doors[1]);
    }

    let print_format = |prefix: &str, v: &Vec::<bool>| {
        let fun = |acc, &x| {
            if x {
                acc + 1
            } else {
                acc
            }
        };
        println!("{}: {:.8}%", prefix, (v.iter().fold(0, fun) as f64 / iterations as f64) * 100_f64);
    };

    print_format("Simple", &simple);
    print_format("Smart", &smart);
}
