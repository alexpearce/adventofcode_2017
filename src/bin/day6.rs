extern crate aoc17;

use aoc17::day6::*;

fn main() {
    let puzzle = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";
    let (solution1, solution2) = iterations_until_cycle(puzzle);
    println!("Puzzle #1: {}", solution1);
    println!("Puzzle #2: {}", solution2);
}
