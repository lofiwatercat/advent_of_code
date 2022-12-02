pub fn process_part1(input: &str) -> String {
    let total_points: u32 = input
        .lines()
        .map(|round| {
            let seperated_round: Vec<&str> = round.split(" ").collect();

            evaluate_round(seperated_round[0], seperated_round[1])
        })
        .sum();

    total_points.to_string()
}

pub fn process_part2(input: &str) -> String {
    let total_points: u32 = input
        .lines()
        .map(|round| {
            let seperated_round: Vec<&str> = round.split(" ").collect();
            evaluate_round2(seperated_round[0], seperated_round[1])
        })
        .sum();

    total_points.to_string()
}

fn evaluate_round(opponent: &str, choice: &str) -> u32 {
    let mut points: u32 = 0;

    match choice {
        "X" => {
            points += 1;
            match opponent {
                "A" => points += 3,
                "B" => points += 0,
                "C" => points += 6,
                _ => (),
            }
        }
        "Y" => {
            points += 2;
            match opponent {
                "A" => points += 6,
                "B" => points += 3,
                "C" => points += 0,
                _ => (),
            }
        }
        "Z" => {
            points += 3;
            match opponent {
                "A" => points += 0,
                "B" => points += 6,
                "C" => points += 3,
                _ => (),
            }
        }
        _ => (),
    }

    points
}

fn evaluate_round2(opponent: &str, choice: &str) -> u32 {
    let mut points: u32 = 0;

    match choice {
        "X" => match opponent {
            "A" => points += 3,
            "B" => points += 1,
            "C" => points += 2,
            _ => (),
        },
        "Y" => {
            points += 3;
            match opponent {
                "A" => points += 1,
                "B" => points += 2,
                "C" => points += 3,
                _ => (),
            }
        }
        "Z" => {
            points += 6;
            match opponent {
                "A" => points += 2,
                "B" => points += 3,
                "C" => points += 1,
                _ => (),
            }
        }
        _ => (),
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn round_evaluation() {
        let result = evaluate_round("A", "Y");
        assert_eq!(result, 8);
    }

    #[test]
    fn round_evaluation2() {
        let result = evaluate_round2("A", "Y");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn test_part_2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
