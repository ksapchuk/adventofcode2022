use regex::Regex;
use std::fs;

fn part1(input: String, y: i64) -> u64 {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut ranges = Vec::new();
    for row in input.trim().lines() {
        if row.len() == 0 {
            continue;
        }
        let cap = re.captures(row.trim()).unwrap();
        let s = (
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
        );
        let b = (
            cap[3].parse::<i64>().unwrap(),
            cap[4].parse::<i64>().unwrap(),
        );
        let dist = (s.0 - b.0).abs() + (s.1 - b.1).abs();
        println!("s: {:?} b: {:?} distance: {}", s, b, dist);

        // distance from sensor to y
        let offset = dist - (y - s.1).abs();
        if offset < 0 {
            println!("Skipping sensor due to distance from y {}", offset);
            continue;
        }
        let range = (s.0 - offset, s.0 + offset);
        println!("Offset: {} Range: {:?}", offset, range);
        ranges.push(range);
    }
    ranges.sort();
    println!("{:?}", ranges);
    let mut sum = 0;
    let mut curr = ranges[0].0;
    for range in ranges {
        if range.1 <= curr {
            continue;
        }
        if range.0 > curr {
            curr = range.0;
        }
        sum += (range.1 - curr).abs();
        println!("curr: {} range:{:?}, sum {}", curr, range, sum);
        curr = range.1;
    }
    return sum.try_into().unwrap();
}

fn part2(input: String, max_bounds: u64) -> u64 {
    return 0;
}

fn main() {
    let input = fs::read_to_string("input/day15.txt").expect("Failed to read file");
    println!("{}", part1(input.clone(), 2000000));
    println!("{}", part2(input.clone(), 4000000));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const TEST_INPUT: &str = "
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    ";
    // #[test]
    fn example1() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part1(input, 10), 26);
    }

    #[test]
    fn example2() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part2(input, 20), 56000011);
    }
}
