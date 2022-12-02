use day_02::process_part1;
use std::fs;

fn main() {
    let strategy_guide = fs::read_to_string("../../input.txt").unwrap();
    let result = process_part1(&strategy_guide);
    println!("Part 1: {}", result);
}
