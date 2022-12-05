use day_03::process_part1;
use std::fs;

fn main() {
    let rucksack_contents = fs::read_to_string("../../input.txt").unwrap();
    let result = process_part1(&rucksack_contents);
    println!("part 1: {}", result);
}
