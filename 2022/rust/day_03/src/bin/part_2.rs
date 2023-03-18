use day_03::process_part2;
use std::fs;

fn main() {
    let rucksack_contents = fs::read_to_string("../../input.txt").unwrap();
    let result = process_part2(&rucksack_contents);
    println!("part 2: {}", result);
}
