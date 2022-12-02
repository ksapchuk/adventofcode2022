use std::fs;

fn part1(input: String) -> u64 {
    let max_sum: u64 = input
        .split("\n\n")
        .map(|elf| {
            return elf
                .trim()
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum();
        })
        .max()
        .unwrap();
    return max_sum;
}

fn part2(input: String) -> u64 {
    let mut calories: Vec<u64> = input
        .split("\n\n")
        .map(|elf| {
            return elf
                .trim()
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum();
        })
        .collect();
    calories.sort();
    calories.reverse();
    return calories[..3].iter().sum::<u64>();
}

fn main() {
    let input = fs::read_to_string("input/day1.txt").expect("Failed to read file");
    println!("{}", part1(input.clone()));
    println!("{}", part2(input.clone()));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const TEST_INPUT: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
    ";
    #[test]
    fn example1() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn example2() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part2(input), 45000);
    }
}
