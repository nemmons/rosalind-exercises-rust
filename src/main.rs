mod problems;

use std::env;
use problems::{dna};

// problem-selection code heavily inspired by https://github.com/agubelu/AoC-rust-template/blob/master/src/main.rs
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }
    let problem = args[1].as_str();
    let solution = solve_problem(problem);
    println!("\n=== Problem {} ===", problem);
    println!("Solution: {}", solution);
}

fn solve_problem(problem: &str) -> String {
    match problem {
        "dna" => dna::solve(),
        _ => unimplemented!()
    }
}