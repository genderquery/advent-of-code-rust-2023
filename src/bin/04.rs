advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    num: usize,
    winning: Vec<u32>,
    picked: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        let mut points = 0;
        for num in self.picked.iter() {
            if self.winning.contains(num) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }

    pub fn matches(&self) -> usize {
        self.picked
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    }
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
            let winning: Vec<_> = winning
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let picked: Vec<_> = picked
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            Card {
                num,
                winning,
                picked,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let mut sum = 0;
    for card in parse_cards(input) {
        sum += card.points();
    }

    sum.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let cards = parse_cards(input);
    let mut cards: Vec<(usize, usize)> = cards
        .iter()
        .map(|card| (card.num, card.matches()))
        .collect();
    let mut pos = 0;
    while pos < cards.len() {
        for i in 0..cards[pos].1 {
            cards.push(cards[cards[pos].0 + i]);
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
