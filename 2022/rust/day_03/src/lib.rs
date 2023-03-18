use std::collections::HashMap;

pub fn process_part1(input: &str) -> u32 {
    let result = input.lines().map(|line| evaluate_contents(line)).sum();

    result
}

pub fn process_part2(input: &str) -> u32 {
    let mut i = 0;
    let mut triplets = String::from("");
    let mut sum = 0;
    for line in input.lines() {
        triplets = format!("{}{}{}", triplets, line, "\n");
        i += 1;
        if i == 3 {
            sum += evaluate_triplets(&triplets);
            i = 0;
            triplets = String::from("");
        }
    }

    sum
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

fn evaluate_triplets(input: &String) -> u32 {
    let mut prio: HashMap<String, u32> = HashMap::new();
    let mut char_counter: HashMap<String, u32> = HashMap::new();
    let mut prio_value = String::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for (i, char) in (0u32..).zip(alphabet.chars()) {
        prio.insert(char.to_string(), i + 1);
    }

    let mut i = 0;
    for line in input.lines() {
        for char in line.chars() {
            char_counter
                .entry(char.to_string())
                .and_modify(|count| {
                    if *count == i {
                        *count += 1;
                    }
                })
                .or_insert(if i == 0 { 1 } else { 0 });
        }
        i += 1;
    }

    for (key, value) in char_counter.iter() {
        if *value == 3 {
            prio_value = key.clone();
        }
    }

    println!("char counter: {:?}", char_counter);
    println!("Triplets: \n{}", input);
    println!("PrioValue: {}", prio_value);

    prio.get(&prio_value).copied().unwrap()
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
        let result = evaluate_triplets(&String::from(input));
        assert_eq!(result, 18);
    }

    #[test]
    fn test_evaluate_triplets2() {
        let input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = evaluate_triplets(&String::from(input));
        assert_eq!(result, 52);
    }

    #[test]
    fn part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let mut sum = 0;
        let mut i = 0;
        let mut triplets = String::new();
        for line in input.lines() {
            triplets = format!("{}{}{}", triplets, line, "\n");
            println!("triplets!: {}", triplets);
            i += 1;
            if i == 3 {
                sum += evaluate_triplets(&triplets);
                i = 0;
                triplets = String::from("");
            }
        }
        assert_eq!(sum, 70);
    }

    #[test]
    fn part_1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 157);
    }
}
