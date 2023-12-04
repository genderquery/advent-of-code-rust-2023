advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy)]
struct Card {
    num: u32,
    points: u32,
    matches: u32,
}

fn card_points(winning: &[&str], picked: &[&str]) -> u32 {
    let matches = card_matches(winning, picked);
    if matches > 0 {
        u32::pow(2, matches - 1)
    } else {
        0
    }
}

fn card_matches(winning: &[&str], picked: &[&str]) -> u32 {
    picked.iter().filter(|num| winning.contains(num)).count() as u32
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (num, numbers) = line.split_once(':').unwrap();
            let num = num
                .strip_prefix("Card")
                .unwrap()
                .trim_start()
                .parse()
                .unwrap();
            let (winning, picked) = numbers.split_once('|').unwrap();
            let winning: Vec<_> = winning.split_ascii_whitespace().collect();
            let picked: Vec<_> = picked.split_ascii_whitespace().collect();
            let points = card_points(&winning, &picked);
            let matches = card_matches(&winning, &picked);
            Card {
                num,
                points,
                matches,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    parse_cards(input)
        .iter()
        .map(|card| card.points)
        .sum::<u32>()
        .to_string()
        .into()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cards = parse_cards(input);
    let mut pos = 0;
    while pos < cards.len() {
        for i in 0..cards[pos].matches {
            let index = (cards[pos].num + i) as usize;
            cards.push(cards[index]);
        }
        pos += 1;
    }

    cards.len().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "30");
    }
}
