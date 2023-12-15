use itertools::Itertools;

use crate::error::AocError;
use nom::{
    bytes::complete::{tag, take_while},
    multi::separated_list1,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        tag(","),
        take_while(|c: char| c.is_alphanumeric() || c == '=' || c == '-'),
    )(input)
}

#[tracing::instrument]
fn hash_line(line: &str) -> usize {
    let ascii_codes = line.chars().map(|c| c as u8).collect_vec();

    let mut sum: usize = 0;

    for code in ascii_codes {
        sum += code as usize;
        sum *= 17;
        sum %= 256;
    }

    sum
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, parsed) = parse(input).unwrap();
    debug_assert!(input.is_empty(), "Failed to completely parse input");

    Ok(parsed.iter().map(|&l| hash_line(l) as u64).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_line(#[case] input: &str, #[case] output: usize) {
        assert_eq!(hash_line(input), output);
    }
    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(1320, process(input)?);
        Ok(())
    }
}
