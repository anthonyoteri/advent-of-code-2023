use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser as _,
};
use nom_supreme::ParserExt as _;
use tracing::instrument;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn distance(&self) -> Vec<u64> {
        (0..self.time)
            .map(|t| t * (self.time - t))
            .collect::<Vec<u64>>()
    }
}

fn nums(input: &str) -> IResult<&str, Vec<u64>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)
}

fn parse_times(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, (times, records)) = separated_pair(nums, line_ending, nums).parse(input)?;

    let races = itertools::izip!(&times, &records)
        .map(|(&time, &record)| Race { time, record })
        .collect();

    Ok((input, races))
}

#[instrument]
fn part_1(input: &str) -> usize {
    let (_, races) = parse_times(input).unwrap();

    let results = races
        .iter()
        .map(|r| {
            r.distance()
                .into_iter()
                .filter(|&d| d > r.record)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    results.iter().map(Vec::len).product()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_part_1() {
        tracing::info!("Hello world");
        let input = include_str!("../test-input.txt");
        assert_eq!(part_1(input), 288);
    }
}
