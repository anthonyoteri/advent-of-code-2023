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
                    Self::FourKind(input.to_string())
                } else {
                    Self::FullHouse(input.to_string())
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
                    Self::ThreeKind(input.to_string())
                } else {
                    Self::TwoPairs(input.to_string())
                }
            }
            4 => Self::OnePair(input.to_string()),
            5 => Self::HighCard(input.to_string()),
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
        'J' => 10 * pos,
        'T' => 9 * pos,
        '9' => 8 * pos,
        '8' => 7 * pos,
        '7' => 6 * pos,
        '6' => 5 * pos,
        '5' => 4 * pos,
        '4' => 3 * pos,
        '3' => 2 * pos,
        '2' => pos,
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
    fn test_string_to_hand() {
        assert_eq!(Hand::OnePair("32T3K".into()), Hand::from("32T3K"));
        assert_eq!(Hand::TwoPairs("KK677".into()), Hand::from("KK677"));
        assert_eq!(Hand::TwoPairs("KTJJT".into()), Hand::from("KTJJT"));
        assert_eq!(Hand::ThreeKind("T55J5".into()), Hand::from("T55J5"));
        assert_eq!(Hand::ThreeKind("QQQJA".into()), Hand::from("QQQJA"));
        assert_eq!(Hand::FullHouse("23332".into()), Hand::from("23332"));
        assert_eq!(Hand::FourKind("AA8AA".into()), Hand::from("AA8AA"));
        assert_eq!(Hand::FiveKind("AAAAA".into()), Hand::from("AAAAA"));
    }

    #[test_log::test]
    fn test_hand_strenght() {
        assert!(Hand::from("33332").strength() > Hand::from("2AAAA").strength());
        assert!(Hand::from("77888").strength() > Hand::from("77788").strength());
        assert!(Hand::from("QQQJA").strength() > Hand::from("T55J5").strength());
        assert!(Hand::from("KK677").strength() > Hand::from("32T3K").strength());
        assert!(Hand::from("22345").strength() > Hand::from("KQJT9").strength());
    }

    #[test_log::test]
    fn test_parse_line() {
        assert_eq!(
            (Hand::from("AAAAA"), 123),
            parse_line("AAAAA 123").unwrap().1
        );
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(6440, process(input)?);
        Ok(())
    }
}
