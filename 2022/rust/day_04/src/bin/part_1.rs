use day_04::process_part1;
use std::fs;

fn main() {
    let assignment_list = fs::read_to_string("input.txt").unwrap();
    let result = process_part1(&assignment_list);
    println!("part 1: {}", result);
}
