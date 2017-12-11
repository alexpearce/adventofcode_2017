extern crate aoc17;

use aoc17::day3::*;

fn main() {
    let puzzle = 265149;
    let (x, y) = spiral_coordinates(puzzle);
    let z = first_spiral_number_larger_than(puzzle);
    println!("Puzzle #1: {}", x.abs() + y.abs());
    println!("Puzzle #2: {}", z);
}
