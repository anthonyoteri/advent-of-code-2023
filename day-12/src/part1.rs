use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::{self, terminated},
    IResult,
};

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

pub fn gen_permutations(s: &mut String, index: usize) -> Vec<String> {
    if index == s.len() {
        return vec![s.clone()];
    }

    let mut permutations = Vec::new();

    if s.chars().nth(index) == Some('?') {
        s.replace_range(index..=index, ".");
        permutations.extend(gen_permutations(s, index + 1));

        s.replace_range(index..=index, "#");
        permutations.extend(gen_permutations(s, index + 1));

        s.replace_range(index..=index, "?");
    } else {
        permutations.extend(gen_permutations(s, index + 1));
    }

    permutations
}

fn make_regex(limits: Vec<usize>) -> String {
    let mut r = String::from(r"^\.*");

    let middle: String = limits
        .iter()
        .map(|n| {
            let mut s = String::from(r"#{");
            s.push_str(&format!("{}", n));
            s.push('}');
            s
        })
        .collect::<Vec<String>>()
        .join(r"\.+");

    r.push_str(&middle);
    r.push_str(r"\.*$");

    r
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, parsed) = parser(input).unwrap();

    Ok(parsed
        .into_iter()
        .map(|(s, limits)| {
            let mut s = String::from(s);
            let re = regex::Regex::new(&make_regex(limits)).unwrap();

            let length = gen_permutations(&mut s, 0)
                .iter()
                .filter(|s| re.is_match(s))
                .count();

            Ok(length)
        })
        .collect::<Result<Vec<usize>, AocError>>()?
        .iter()
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test]
    fn test_parser() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3";
        let result = parser(input);
        assert!(result.is_ok());

        let (rest, result) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "???.###");
        assert_eq!(result[0].1, vec![1, 1, 3]);
        assert_eq!(result[1].0, ".??..??...?##.");
        assert_eq!(result[1].1, vec![1, 1, 3]);
    }

    #[test_log::test(rstest)]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    fn test_length(#[case] input: &str, #[case] expected: usize) {
        let mut input = String::from(input);

        let re = regex::Regex::new(r"^\.*#{1}\.+#{1}\.+#{3}\.*\s").unwrap();

        let length = gen_permutations(&mut input, 0)
            .iter()
            .filter(|s| re.is_match(s))
            .count();

        assert_eq!(length, expected);
    }

    #[test_log::test]
    fn test_make_regex() {
        let limits = vec![1, 2, 3, 4];
        let regex = make_regex(limits);
        assert_eq!(regex, r"^\.*#{1}\.+#{2}\.+#{3}\.+#{4}\.*$");
    }

    #[test_log::test(rstest)]
    #[case(("???.###", vec![1,1,3]), 1)]
    #[case((".??..??...?##.", vec![1,1,3]), 4)]
    #[case(("?###????????", vec![3,2,1]), 10)]
    fn test_regex(#[case] params: (&str, Vec<usize>), #[case] expected: usize) {
        let mut input = String::from(params.0);
        let limits = params.1;

        let re = regex::Regex::new(&make_regex(limits)).unwrap();

        let length = gen_permutations(&mut input, 0)
            .iter()
            .filter(|s| re.is_match(s))
            .count();

        assert_eq!(length, expected);
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(21, process(input)?);
        Ok(())
    }
}
