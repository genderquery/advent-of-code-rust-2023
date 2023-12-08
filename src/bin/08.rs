use std::collections::HashMap;

advent_of_code::solution!(8);

enum Direction {
    Left,
    Right,
}

type Instructions = Vec<Direction>;
type Network = HashMap<String, (String, String)>;

fn parse(input: &str) -> (Instructions, Network) {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("invalid direction"),
        })
        .collect();
    lines.next().unwrap();
    let network = lines
        .map(|line| {
            let entry = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (entry, (left, right))
        })
        .collect();
    (instructions, network)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (instructions, network) = parse(input);

    let mut steps = 0;
    let mut node = "AAA";
    for instruction in instructions.iter().cycle() {
        let (left, right) = network.get(node).unwrap();
        match instruction {
            Direction::Left => node = left,
            Direction::Right => node = right,
        }
        steps += 1;
        if node == "ZZZ" {
            break;
        }
    }

    steps.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let (instructions, network) = parse(input);

    let nodes: Vec<_> = network.keys().filter(|node| node.ends_with('A')).collect();
    let mut all_steps = Vec::new();
    for node in nodes.iter() {
        let mut steps = 0;
        let mut position = node.to_owned();
        for instruction in instructions.iter().cycle() {
            let (left, right) = network.get(position).unwrap();
            match instruction {
                Direction::Left => position = left,
                Direction::Right => position = right,
            }
            steps += 1;
            if position.ends_with('Z') {
                all_steps.push(steps);
                break;
            }
        }
    }

    let lcm = all_steps.iter().fold(1, |acc, x| lcm(acc, *x));
    lcm.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        let input: &str = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};
        let result = part_one(input).unwrap();
        assert_eq!(result, "2");
        let input: &str = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};
        let result = part_one(input).unwrap();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part_two() {
        let input: &str = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        let result = part_two(input).unwrap();
        assert_eq!(result, "6");
    }
}
