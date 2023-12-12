use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::{self, terminated},
    IResult,
};
use rayon::prelude::*;

use crate::error::AocError;

fn parse_input(input: &str) -> IResult<&str, &str> {
    terminated(take_until(" "), space1)(input)
}

fn parse_limits(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, limits) = separated_list1(tag(","), complete::u32)(input)?;
    Ok((input, limits.into_iter().map(|n| n as usize).collect()))
}

fn parser(input: &str) -> IResult<&str, Vec<(&str, Vec<usize>)>> {
    many1(terminated(
        sequence::tuple((parse_input, parse_limits)),
        alt((line_ending, eof)),
    ))(input)
}

#[memoize::memoize]
fn count(input: String, limits: Vec<usize>) -> usize {
    if input.is_empty() {
        if limits.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if limits.is_empty() {
        if input.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }
    let mut n = 0;

    if input.starts_with('.') || input.starts_with('?') {
        n += count(input[1..].to_string(), limits.clone());
    }
    if (input.starts_with('#') || input.starts_with('?')) && limits[0] <= input.len()
            && !input.get(..limits[0]).unwrap().contains('.') && (limits[0] == input.len() || input.chars().nth(limits[0]).unwrap() != '#') {
        n += count(
            input.get(limits[0] + 1..).unwrap_or_default().to_string(),
            limits[1..].to_vec(),
        );
    }
    n
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, parsed) = parser(input).unwrap();

    Ok(parsed
        .par_iter()
        .map(|(string, limits)| {
            let string = std::iter::repeat(string)
                .take(5)
                .cloned()
                .collect::<Vec<&str>>()
                .join("?");
            let limits = limits
                .iter()
                .cycle()
                .take(5 * limits.len())
                .cloned()
                .collect::<Vec<_>>();

            count(string.clone(), limits)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(525152, process(input)?);
        Ok(())
    }
}
