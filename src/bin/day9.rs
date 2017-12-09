extern crate aoc17;

use std::fs::File;
use std::io::prelude::*;

use aoc17::day9::*;

fn main() {
    let mut f = File::open("input/day9.txt").expect("Could not open input");
    let mut puzzle = String::new();
    f.read_to_string(&mut puzzle).expect("Could not read input");

    let (solution1, solution2) = stream_score(&puzzle);
    println!("Puzzle #1: {}", solution1);
    println!("Puzzle #2: {}", solution2);
}

