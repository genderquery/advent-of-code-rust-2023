advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn predict_1(seq: &[i32]) -> i32 {
    let new_seq: Vec<i32> = seq.windows(2).map(|w| w[1] - w[0]).collect();
    if new_seq.iter().all(|e| *e == 0) {
        seq[0]
    } else {
        seq.last().unwrap() + predict_1(&new_seq)
    }
}

fn predict_2(seq: &[i32]) -> i32 {
    let new_seq: Vec<i32> = seq.windows(2).rev().map(|w| w[0] - w[1]).collect();
    if new_seq.iter().all(|e| *e == 0) {
        seq[0]
    } else {
        seq.first().unwrap() + predict_1(&new_seq)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let sum: i32 = parse(input).iter().map(|h| predict_1(h)).sum();
    sum.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let sum: i32 = parse(input).iter().map(|h| predict_2(h)).sum();
    sum.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "114");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "2");
    }
}
