extern crate aoc17;

use std::fs::File;
use std::io::prelude::*;

use aoc17::day12::*;

fn main() {
    let mut f = File::open("input/day12.txt").expect("Could not open input");
    let mut puzzle = String::new();
    f.read_to_string(&mut puzzle).expect("Could not read input");

    let connected = connected_nodes(&puzzle, 0);
    let disconnected = disconnected_graphs(&puzzle);
    println!("Puzzle #1: {}", connected.len());
    println!("Puzzle #2: {}", disconnected.len());
}

