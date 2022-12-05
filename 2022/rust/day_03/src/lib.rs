use std::collections::HashMap;

pub fn process_part1(input: &str) -> u32 {
    let result = input.lines().map(|line| evaluate_contents(line)).sum();

    result
}

pub fn process_part2(input: &str) -> u32 {
    let input_to_parse = input.lines();

    0
}

fn evaluate_contents(line: &str) -> u32 {
    let mut prio: HashMap<String, u32> = HashMap::new();
    let mut char_counter: HashMap<String, u32> = HashMap::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut prio_char = "".to_string();

    // Add the alphabet and their priority values into the prio hashmap
    for (i, char) in (0u32..).zip(alphabet.chars()) {
        prio.insert(char.to_string(), i + 1);
    }

    let len = line.len();

    let first_half = &line[0..(len / 2)];
    let second_half = &line[(len / 2)..];

    // iterate through the first half and increment char_counter
    for char in first_half.chars() {
        let count = char_counter
            .entry(char.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // Iterate throught the second half, and if it finds any non-zero characters in the
    // char counter, then that is the priority
    for char in second_half.chars() {
        let count = char_counter.entry(char.to_string()).or_insert(0);
        if *count != 0 {
            prio_char = char.to_string();
        }
    }

    prio.get(&prio_char).copied().unwrap()
}

fn evaluate_triplets(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn test_evaluate_contents() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let result = evaluate_contents(input);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_evaluate_triplets() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        let result = evaluate_triplets(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn part_1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 157);
    }
}
