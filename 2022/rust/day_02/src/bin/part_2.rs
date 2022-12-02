use day_02::process_part2;
use std::fs;

fn main() {
    let strategy_guide = fs::read_to_string("../../input.txt").unwrap();
    let result = process_part2(&strategy_guide);

    println!("Part 2: {}", result);
}
