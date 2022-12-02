use day_01::process_part1;
use std::fs;

fn main() {
    let calorie_log = fs::read_to_string("../../input.txt").unwrap();
    let part1 = process_part1(&calorie_log);
    println!("part1: {}", part1);
}
