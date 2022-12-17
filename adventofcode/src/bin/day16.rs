use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Valve {
    name: String,
    rate: i64,
    paths: Vec<String>,
}

fn dfs(
    name: &String,
    map: &HashMap<String, Valve>,
    visited: Vec<String>,
    remaining: i64,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if remaining <= 0 {
        return 0;
    }

    let key = name.clone() + &visited.join("") + &remaining.to_string();
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    let v = map.get(name).unwrap();
//     println!(
//         "Remaining {} Valve: {:?} Opened: {:?}",
//         remaining, v, visited
//     );

    let mut best = 0;

    for path in &v.paths {
        let res = dfs(path, map, visited.clone(), remaining - 1, memo);
        if res > best {
            best = res;
        }
    }

    if !visited.contains(name) && v.rate > 0 {
        let new_flow = v.rate * (remaining - 1);
        let mut new_visited = visited.clone();
        new_visited.push(name.clone());

        for path in &v.paths {
            let res = new_flow + dfs(path, map, new_visited.clone(), remaining - 2, memo);
            if res > best {
                best = res;
            }
        }
    }
    memo.insert(key, best);
    return best;
}

fn part1(input: String) -> i64 {
    let re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w+,?\s?)+)")
            .unwrap();
    let mut valves = HashMap::new();
    for row in input.trim().lines() {
        if row.trim().len() == 0 {
            continue;
        }
        let cap = re.captures(row.trim()).unwrap();
        let v = Valve {
            name: cap[1].to_string(),
            rate: cap[2].parse().unwrap(),
            paths: cap[3].split(',').map(|s| s.trim().to_string()).collect(),
        };
        valves.insert(v.name.clone(), v);
    }
    println!("{:?}", valves);
    let mut memo = HashMap::new();
    let res = dfs(&"AA".to_string(), &valves, Vec::new(), 30, &mut memo);
//     println!("{:?}", memo);
    return res;
}

fn part2(input: String) -> i64 {
    return 0;
}

fn main() {
    let input = fs::read_to_string("input/day16.txt").expect("Failed to read file");
    println!("{}", part1(input.clone()));
    println!("{}", part2(input.clone()));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    const TEST_INPUT: &str = "
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II
    ";
    #[test]
    fn example1() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part1(input), 1651);
    }

    //     #[test]
    fn example2() {
        let input = TEST_INPUT.to_string();
        assert_eq!(part2(input), 56000011);
    }
}
