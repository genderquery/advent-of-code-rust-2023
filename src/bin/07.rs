use std::{cmp::Ordering, collections::HashMap, str::FromStr};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Ace = 15,
    King = 14,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand<const J_IS_WILD: bool>(Vec<Card>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum HandKind {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl<const J_IS_WILD: bool> Hand<J_IS_WILD> {
    fn kind(&self) -> HandKind {
        let mut count = HashMap::new();
        for card in self.0.iter() {
            count.entry(*card).and_modify(|x| *x += 1).or_insert(1);
        }
        if J_IS_WILD {
            // Find the most common card that isn't a joker...
            let (most_common, _) = count
                .iter()
                .filter(|(card, _)| **card != Card::Joker)
                .max_by(|(_, a), (_, b)| a.cmp(b))
                // edge case with all jokers
                .unwrap_or((&Card::Joker, &5));
            // Replace all the jokers with the most common card
            let new_hand: Vec<Card> = self
                .0
                .iter()
                .map(|card| {
                    if *card == Card::Joker {
                        *most_common
                    } else {
                        *card
                    }
                })
                .collect();
            // Reevaluate
            count = HashMap::new();
            for card in new_hand.iter() {
                count.entry(*card).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        if count.values().any(|&c| c == 5) {
            HandKind::FiveOfAKind
        } else if count.values().any(|&c| c == 4) {
            HandKind::FourOfAKind
        } else if count.len() == 2 {
            HandKind::FullHouse
        } else if count.values().any(|&c| c == 3) {
            HandKind::ThreeOfAKind
        } else if count.len() == 3 {
            HandKind::TwoPair
        } else if count.len() == 4 {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }
}

impl<const J_IS_WILD: bool> Ord for Hand<J_IS_WILD> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind() == other.kind() {
            // Compare each set of cards
            for (a, b) in self.0.iter().zip(other.0.iter()) {
                // Skip until we find different ranks
                if a == b {
                    continue;
                }
                return a.cmp(b);
            }
            // Fallthrough for same hands
            Ordering::Equal
        } else {
            self.kind().cmp(&other.kind())
        }
    }
}

impl<const J_IS_WILD: bool> PartialOrd for Hand<J_IS_WILD> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const J_IS_WILD: bool> FromStr for Hand<J_IS_WILD> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Result<Vec<Card>, &'static str> = s
            .chars()
            .map(|c| match c {
                'A' => Ok(Card::Ace),
                'K' => Ok(Card::King),
                'Q' => Ok(Card::Queen),
                'J' => {
                    if J_IS_WILD {
                        Ok(Card::Joker)
                    } else {
                        Ok(Card::Jack)
                    }
                }
                'T' => Ok(Card::Ten),
                '9' => Ok(Card::Nine),
                '8' => Ok(Card::Eight),
                '7' => Ok(Card::Seven),
                '6' => Ok(Card::Six),
                '5' => Ok(Card::Five),
                '4' => Ok(Card::Four),
                '3' => Ok(Card::Three),
                '2' => Ok(Card::Two),
                _ => Err("invalid card"),
            })
            .collect();
        Ok(Hand(v?))
    }
}

impl<const J_IS_WILD: bool> ToString for Hand<J_IS_WILD> {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|c| match c {
                Card::Ace => 'A',
                Card::King => 'K',
                Card::Queen => 'Q',
                Card::Jack => 'J',
                Card::Ten => 'T',
                Card::Nine => '9',
                Card::Eight => '8',
                Card::Seven => '7',
                Card::Six => '6',
                Card::Five => '5',
                Card::Four => '4',
                Card::Three => '3',
                Card::Two => '2',
                Card::Joker => 'J',
            })
            .collect()
    }
}

fn parse<const J_IS_WILD: bool>(input: &str) -> Vec<(Hand<J_IS_WILD>, u32)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand.parse().unwrap(), bid.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let mut hands = parse::<false>(input);
    hands.sort();
    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * bid)
        .sum::<u32>();
    sum.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut hands = parse::<true>(input);
    hands.sort();
    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * bid)
        .sum::<u32>();
    sum.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_hand_kind() {
        assert_eq!(
            Hand::<false>::from_str("AAAAA").unwrap().kind(),
            HandKind::FiveOfAKind
        );
        assert_eq!(
            Hand::<false>::from_str("AA8AA").unwrap().kind(),
            HandKind::FourOfAKind
        );
        assert_eq!(
            Hand::<false>::from_str("23332").unwrap().kind(),
            HandKind::FullHouse
        );
        assert_eq!(
            Hand::<false>::from_str("TTT98").unwrap().kind(),
            HandKind::ThreeOfAKind
        );
        assert_eq!(
            Hand::<false>::from_str("23432").unwrap().kind(),
            HandKind::TwoPair
        );
        assert_eq!(
            Hand::<false>::from_str("A23A4").unwrap().kind(),
            HandKind::OnePair
        );
        assert_eq!(
            Hand::<false>::from_str("23456").unwrap().kind(),
            HandKind::HighCard
        );
    }

    #[test]
    fn test_hand_order() {
        let mut hands: Vec<Hand<false>> = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"]
            .iter()
            .map(|h| h.parse().unwrap())
            .collect();
        hands.sort();
        let hands: Vec<String> = hands.iter().map(|h| h.to_string()).collect();
        assert_eq!(hands, vec!["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"]);
    }

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "6440");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "5905");
    }
}
