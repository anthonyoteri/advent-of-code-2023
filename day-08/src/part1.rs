use std::collections::BTreeMap;

use nom::{
    character::complete::{self, alpha1, multispace0, newline, one_of},
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
    let (input, instructions) = many1(one_of("LR"))(input)?;
    Ok((input, instructions))
}

fn parse_entry(input: &str) -> IResult<&str, (String, Entry)> {
    let (input, node) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = terminated(complete::char('='), multispace0)(input)?;
    let (input, (left, right)) = delimited(
        complete::char('('),
        separated_pair(
            alpha1,
            preceded(multispace0, complete::char(',')),
            preceded(multispace0, alpha1),
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
    let (input, _) = newline(input)?;
    let (input, entries) = many1(terminated(parse_entry, newline))(input)?;

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

    let mut directions = instructions.iter().cycle();

    let mut current = "AAA".to_string();
    let mut entry;
    let mut count = 0;
    while current != "ZZZ" {
        entry = entries.get(&current).unwrap();
        let direction = directions.next().unwrap();
        current = match direction {
            'L' => entry.left.clone(),
            'R' => entry.right.clone(),
            _ => unreachable!(),
        };
        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_parse_instructions() {
        let input = "LRLRLR";
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
        let input = include_str!("../test-input.txt");
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
