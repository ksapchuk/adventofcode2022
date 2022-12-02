use std::fs;

fn part1(input: String) -> u64 {
    let mut score = 0;
    for round in input.trim().lines() {
        let mut chars = round.trim().chars();
        let op = chars.nth(0).unwrap();
        let c = chars.nth(1).unwrap();

        let round_score = match op {
            'A' => match c {
                'X' => 1 + 3,
                'Y' => 2 + 6,
                'Z' => 3 + 0,
                _ => 0,
            },
            'B' => match c {
                'X' => 1 + 0,
                'Y' => 2 + 3,
                'Z' => 3 + 6,
                _ => 0,
            },
            'C' => match c {
                'X' => 1 + 6,
                'Y' => 2 + 0,
                'Z' => 3 + 3,
                _ => 0,
            },
            _ => 0,
        };
        score += round_score
    }
    return score;
}

fn part2(input: String) -> u64 {
    let mut score = 0;
    for round in input.trim().lines() {
        let mut chars = round.trim().chars();
        let op = chars.nth(0).unwrap();
        let c = chars.nth(1).unwrap();

        let round_score = match op {
            'A' => match c {
                'X' => 0 + 3,
                'Y' => 3 + 1,
                'Z' => 6 + 2,
                _ => 0,
            },
            'B' => match c {
                'X' => 0 + 1,
                'Y' => 3 + 2,
                'Z' => 6 + 3,
                _ => 0,
            },
            'C' => match c {
                'X' => 0 + 2,
                'Y' => 3 + 3,
                'Z' => 6 + 1,
                _ => 0,
            },
            _ => 0,
        };
        score += round_score
    }
    return score;
}

fn main() {
    let input = fs::read_to_string("input/day2.txt").expect("Failed to read file");
    println!("{}", part1(input.clone()));
    println!("{}", part2(input.clone()));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const TEST_INPUT: &str = "
A Y
B X
C Z
    ";
    #[test]
    fn example1() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn example2() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part2(input), 12);
    }
}
