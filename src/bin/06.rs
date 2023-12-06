advent_of_code::solution!(6);

fn parse_line_1<'a>(input: &'a str, prefix: &str) -> impl Iterator<Item = u64> + 'a {
    input
        .strip_prefix(prefix)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
}

fn parse_1(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let times = parse_line_1(lines.next().unwrap(), "Time:");
    let distances = parse_line_1(lines.next().unwrap(), "Distance:");
    times.zip(distances).collect()
}

fn parse_line_2(input: &str, prefix: &str) -> u64 {
    input
        .strip_prefix(prefix)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn parse_2(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let time = parse_line_2(lines.next().unwrap(), "Time:");
    let distance = parse_line_2(lines.next().unwrap(), "Distance:");
    (time, distance)
}

fn winning_distances(time: u64, record_distance: u64) -> u64 {
    let mut winning_distances = 0;
    for hold_time in 1..time {
        let movement_time = time - hold_time;
        let distance = hold_time * movement_time;
        if distance > record_distance {
            winning_distances += 1;
        }
    }
    winning_distances
}

pub fn part_one(input: &str) -> Option<String> {
    let mut total_wins_product = 1;
    for (time, record_distance) in parse_1(input) {
        total_wins_product *= winning_distances(time, record_distance);
    }
    total_wins_product.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let (time, record_distance) = parse_2(input);
    let winning_distances = winning_distances(time, record_distance);
    winning_distances.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "288");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "71503");
    }
}
