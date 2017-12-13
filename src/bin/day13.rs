extern crate aoc17;

use std::fs::File;
use std::io::prelude::*;

use aoc17::day13::*;

fn main() {
    let mut f = File::open("input/day13.txt").expect("Could not open input");
    let mut puzzle = String::new();
    f.read_to_string(&mut puzzle).expect("Could not read input");

    let firewall = create_firewall(&puzzle);

    println!("Puzzle #1: {}", severity(&firewall, 0));

    let mut delay = 0;
    while !transmitted(&firewall, delay) {
        delay += 1;
    }
    println!("Puzzle #2: {}", delay);
}

