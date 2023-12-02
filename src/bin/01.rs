use aho_corasick::AhoCorasick;

advent_of_code::solution!(1);

fn to_digit(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("not a valid digit"),
    }
}

fn sum_calibration_digits(input: &str, digits: &[&str]) -> u32 {
    let digits = AhoCorasick::new(digits).unwrap();
    input
        .lines()
        .map(|line| {
            let matches_iter = || {
                digits
                    .find_overlapping_iter(line)
                    .map(|mat| &line[mat.start()..mat.end()])
            };
            let first = to_digit(matches_iter().next().unwrap());
            let last = to_digit(matches_iter().last().unwrap());
            first * 10 + last
        })
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<String> {
    sum_calibration_digits(input, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .to_string()
        .into()
}

pub fn part_two(input: &str) -> Option<String> {
    sum_calibration_digits(
        input,
        &[
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three",
            "four", "five", "six", "seven", "eight", "nine",
        ],
    )
    .to_string()
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "142");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
    "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "281");
    }
}
