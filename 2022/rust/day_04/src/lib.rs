pub fn process_part1(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| if is_contained(line) { 1 } else { 0 })
        .sum();

    result
}

pub fn process_part2(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            if is_contained(line) || is_overlap(line) {
                1
            } else {
                0
            }
        })
        .sum();

    result
}

fn left_n_right(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut split = input.split(",");
    let left = split.next().unwrap();
    let right = split.next().unwrap();

    let left_split: Vec<u32> = left
        .split("-")
        .map(|str| str.parse::<u32>().unwrap())
        .collect();
    let right_split: Vec<u32> = right
        .split("-")
        .map(|str| str.parse::<u32>().unwrap())
        .collect();

    (left_split, right_split)
}

fn is_contained(input: &str) -> bool {
    let (left_split, right_split) = left_n_right(input);
    if left_split[0] <= right_split[0] && left_split[1] >= right_split[1] {
        if left_split[0] <= right_split[0] {}
        return true;
    }

    if right_split[0] <= left_split[0] && right_split[1] >= left_split[1] {
        return true;
    }

    false
}

fn is_overlap(input: &str) -> bool {
    let (left_split, right_split) = left_n_right(input);
    if left_split[0] <= right_split[1] && left_split[0] >= right_split[0] {
        return true;
    }
    if right_split[0] <= left_split[1] && right_split[0] >= left_split[0] {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_is_contained() {
        let result: u32 = INPUT
            .lines()
            .map(|line| if is_contained(line) { 1 } else { 0 })
            .sum();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_is_overlap() {
        let result: u32 = INPUT
            .lines()
            .map(|line| {
                if is_contained(line) || is_overlap(line) {
                    1
                } else {
                    0
                }
            })
            .sum();

        assert_eq!(result, 4);
    }
}
