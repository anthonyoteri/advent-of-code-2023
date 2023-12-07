use nom::{
    character::complete::{self, alphanumeric1, newline, space1},
    IResult,
};

use crate::error::AocError;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Hand {
    HighCard(String),
    OnePair(String),
    TwoPairs(String),
    ThreeKind(String),
    FullHouse(String),
    FourKind(String),
    FiveKind(String),
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        let chars = input.chars().collect::<BTreeSet<char>>();

        let num_jokers = input.chars().filter(|c| *c == 'J').count();

        match chars.len() {
            1 => Self::FiveKind(input.to_string()),
            2 => {
                let mut iter = chars.iter();
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                let mut first_count = 0;
                let mut second_count = 0;
                for c in input.chars() {
                    if c == *first {
                        first_count += 1;
                    } else if c == *second {
                        second_count += 1;
                    }
                }
                if first_count == 4 || second_count == 4 {
                    match num_jokers {
                        1 | 4 => Self::FiveKind(input.to_string()),
                        _ => Self::FourKind(input.to_string()),
                    }
                } else {
                    match num_jokers {
                        2..=3 => Self::FiveKind(input.to_string()),
                        _ => Self::FullHouse(input.to_string()),
                    }
                }
            }
            3 => {
                let mut iter = chars.iter();
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                let third = iter.next().unwrap();
                let mut first_count = 0;
                let mut second_count = 0;
                let mut third_count = 0;
                for c in input.chars() {
                    if c == *first {
                        first_count += 1;
                    } else if c == *second {
                        second_count += 1;
                    } else if c == *third {
                        third_count += 1;
                    }
                }
                if first_count == 3 || second_count == 3 || third_count == 3 {
                    match num_jokers {
                        1 | 3 => Self::FourKind(input.to_string()),
                        2 => Self::FiveKind(input.to_string()),
                        _ => Self::ThreeKind(input.to_string()),
                    }
                } else {
                    match num_jokers {
                        2 => Self::FourKind(input.to_string()),
                        1 => Self::FullHouse(input.to_string()),
                        _ => Self::TwoPairs(input.to_string()),
                    }
                }
            }
            4 => match num_jokers {
                3 => Self::FiveKind(input.to_string()),
                1..=2 => Self::ThreeKind(input.to_string()),
                _ => Self::OnePair(input.to_string()),
            },
            5 => match num_jokers {
                1 => Self::OnePair(input.to_string()),
                _ => Self::HighCard(input.to_string()),
            },
            _ => panic!("Not implemented"),
        }
    }
}

fn card_value(i: usize, c: char) -> usize {
    let pos = 13_u32.pow(5 - i as u32);

    let v = match c {
        'A' => 13 * pos,
        'K' => 12 * pos,
        'Q' => 11 * pos,
        'T' => 10 * pos,
        '9' => 9 * pos,
        '8' => 8 * pos,
        '7' => 7 * pos,
        '6' => 6 * pos,
        '5' => 5 * pos,
        '4' => 4 * pos,
        '3' => 3 * pos,
        '2' => 2 * pos,
        'J' => pos,
        _ => panic!("Not implemented"),
    };

    v as usize
}

impl Hand {
    fn strength(&self) -> usize {
        match self {
            Self::HighCard(hand) => {
                13_usize.pow(7)
                    + hand
                        .chars()
                        .enumerate()
                        .map(|(i, c)| card_value(i, c))
                        .sum::<usize>()
            }
            Self::OnePair(hand) => {
                13_usize.pow(8)
                    + hand
                        .chars()
                        .enumerate()
                        .map(|(i, c)| card_value(i, c))
                        .sum::<usize>()
            }
            Self::TwoPairs(hand) => {
                13_usize.pow(9)
                    + hand
                        .chars()
                        .enumerate()
                        .map(|(i, c)| card_value(i, c))
                        .sum::<usize>()
            }
            Self::ThreeKind(hand) => {
                13_usize.pow(10)
                    + hand
                        .chars()
                        .enumerate()
                        .map(|(i, c)| card_value(i, c))
                        .sum::<usize>()
            }
            Self::FullHouse(hand) => {
                13_usize.pow(11)
                    + hand
                        .chars()
                        .enumerate()
                        .map(|(i, c)| card_value(i, c))
                        .sum::<usize>()
            }
            Self::FourKind(hand) => {
                13_usize.pow(12)
                    + hand
                        .chars()
                        .enumerate()
                        .map(|(i, c)| card_value(i, c))
                        .sum::<usize>()
            }
            Self::FiveKind(hand) => {
                13_usize.pow(13)
                    + hand
                        .chars()
                        .enumerate()
                        .map(|(i, c)| card_value(i, c))
                        .sum::<usize>()
            }
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, (Hand, u32)> {
    let (input, hand) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = complete::u32(input)?;

    Ok((input, (Hand::from(hand), bid)))
}

fn parse(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
    let (input, hands) = nom::multi::separated_list1(newline, parse_line)(input)?;

    Ok((input, hands))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, mut game) = parse(input).unwrap();

    game.sort_by(|a, b| a.0.strength().cmp(&b.0.strength()));

    let scores = game
        .iter()
        .enumerate()
        .map(|(pos, (_, bid))| *bid as u64 * (1 + pos as u64))
        .collect::<Vec<u64>>();

    Ok(scores.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_hands() {
        let hand1 = Hand::from("32T3K");
        let hand2 = Hand::from("T55J5");
        let hand3 = Hand::from("KK677");
        let hand4 = Hand::from("KTJJT");
        let hand5 = Hand::from("QQQJA");

        assert_eq!(hand1, Hand::OnePair("32T3K".into()));
        assert_eq!(hand3, Hand::TwoPairs("KK677".into()));
        assert_eq!(hand2, Hand::FourKind("T55J5".into()));
        assert_eq!(hand4, Hand::FourKind("KTJJT".into()));
        assert_eq!(hand5, Hand::FourKind("QQQJA".into()));
    }

    #[test_log::test]
    fn test_strength() {
        let hand1 = Hand::from("32T3K");
        let hand2 = Hand::from("T55J5");
        let hand3 = Hand::from("KK677");
        let hand4 = Hand::from("KTJJT");
        let hand5 = Hand::from("QQQJA");

        let mut hands = vec![&hand1, &hand2, &hand3, &hand4, &hand5];
        hands.sort_by_key(|b| std::cmp::Reverse(b.strength()));

        assert_eq!(hands, vec![&hand4, &hand5, &hand2, &hand3, &hand1]);
    }

    #[test_log::test]
    fn test_upgrades_1_distinct() {
        assert!(matches!(Hand::from("KKKKK"), Hand::FiveKind(_)));
        assert!(matches!(Hand::from("JJJJJ"), Hand::FiveKind(_)));
    }

    #[test_log::test]
    fn test_upgrades_2_distinct() {
        assert!(matches!(Hand::from("KKKKQ"), Hand::FourKind(_)));
        assert!(matches!(Hand::from("KKKKJ"), Hand::FiveKind(_)));
        assert!(matches!(Hand::from("KKKJJ"), Hand::FiveKind(_)));
        assert!(matches!(Hand::from("KKJJJ"), Hand::FiveKind(_)));
        assert!(matches!(Hand::from("KJJJJ"), Hand::FiveKind(_)));
    }

    #[test_log::test]
    fn test_upgrades_3_distinct() {
        assert!(matches!(Hand::from("QKKKT"), Hand::ThreeKind(_)));
        assert!(matches!(Hand::from("QKKKJ"), Hand::FourKind(_)));
        assert!(matches!(Hand::from("QKKJJ"), Hand::FourKind(_)));
        assert!(matches!(Hand::from("QKJJJ"), Hand::FourKind(_)));
    }

    #[test_log::test]
    fn test_upgrades_4_distinct() {
        assert!(matches!(Hand::from("QTKK9"), Hand::OnePair(_)));
        assert!(matches!(Hand::from("QTKKJ"), Hand::ThreeKind(_)));
        assert!(matches!(Hand::from("QTKJJ"), Hand::ThreeKind(_)));
    }

    #[test_log::test]
    fn test_upgrades_5_distinct() {
        assert!(matches!(Hand::from("QT9K8"), Hand::HighCard(_)));
        assert!(matches!(Hand::from("QT9KJ"), Hand::OnePair(_)));
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(5905, process(input)?);
        Ok(())
    }
}
