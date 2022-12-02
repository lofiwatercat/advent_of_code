use day_01::process_part2;
use std::fs;

fn main() {
    let calorie_log = fs::read_to_string("../../input.txt").unwrap();
    let part2 = process_part2(&calorie_log);
    println!("part2: {}", part2);
}
