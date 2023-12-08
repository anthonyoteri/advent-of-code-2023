use std::collections::BTreeMap;

use nom::{
    branch::alt,
    character::complete::{self, alphanumeric1, line_ending, multispace0, newline, one_of},
    combinator::eof,
    multi::many1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::error::AocError;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Entry {
    left: String,
    right: String,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<char>> {
    let (input, instructions) = terminated(many1(one_of("LR")), newline)(input)?;
    Ok((input, instructions))
}

fn parse_entry(input: &str) -> IResult<&str, (String, Entry)> {
    let (input, node) = alphanumeric1(input)?;
    let (input, _) = terminated(preceded(multispace0, complete::char('=')), multispace0)(input)?;
    let (input, (left, right)) = delimited(
        complete::char('('),
        separated_pair(
            alphanumeric1,
            preceded(multispace0, complete::char(',')),
            preceded(multispace0, alphanumeric1),
        ),
        complete::char(')'),
    )(input)?;
    Ok((
        input,
        (
            node.to_string(),
            Entry {
                left: left.to_string(),
                right: right.to_string(),
            },
        ),
    ))
}

fn parse(input: &str) -> IResult<&str, (Vec<char>, BTreeMap<String, Entry>)> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = newline(input)?;
    let (input, entries) = many1(terminated(parse_entry, alt((line_ending, eof))))(input)?;

    debug_assert!(dbg!(&input).is_empty());

    Ok((
        input,
        (
            instructions,
            entries.into_iter().collect::<BTreeMap<String, Entry>>(),
        ),
    ))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, (instructions, entries)) = parse(input).unwrap();
    let current = entries
        .keys()
        .filter(|k| k.ends_with('Z'))
        .cloned()
        .collect::<Vec<_>>();

    let _count = 0;

    let results = current
        .iter()
        .map(|node| {
            let mut visited = vec![node.clone()];
            let mut current = node.clone();

            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let entry = entries.get(&current).unwrap();
                    let next_node = match instruction {
                        'L' => &entry.left,
                        'R' => &entry.right,
                        _ => unreachable!(),
                    };

                    if next_node.ends_with('Z') {
                        Some(index + 1)
                    } else {
                        visited.push(next_node.clone());
                        current = next_node.clone();
                        None
                    }
                })
                .unwrap()
        })
        .collect::<Vec<usize>>();

    Ok(lcm(&results) as u64)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_instructions() {
        let input = "LRLRLR\n";
        assert_eq!(
            parse_instructions(input),
            Ok(("", vec!['L', 'R', 'L', 'R', 'L', 'R']))
        );
    }

    #[test_log::test]
    fn test_parse_entry() {
        let input = "AAA = (BBB, CCC)";

        assert_eq!(
            parse_entry(input),
            Ok((
                "",
                (
                    "AAA".to_string(),
                    Entry {
                        left: "BBB".to_string(),
                        right: "CCC".to_string(),
                    }
                )
            ))
        );
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input2.txt");
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
