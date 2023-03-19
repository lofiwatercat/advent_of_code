use day_04::process_part2;
use std::fs;

fn main() {
    let assignment_list = fs::read_to_string("input.txt").unwrap();
    let result = process_part2(&assignment_list);
    println!("part 2: {}", result);
}
