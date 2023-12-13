use crate::error::AocError;
use itertools::Itertools as _;
use nom::character::complete::line_ending;
use nom::{
    bytes::complete::is_a, character::complete::newline, multi::separated_list1, sequence::pair,
    IResult,
};
use tracing::instrument;

#[instrument(skip(input))]
fn block(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, is_a(".#"))(input)
}

#[instrument(skip(input))]
fn parse(input: &str) -> IResult<&str, Vec<(Vec<String>, Vec<String>)>> {
    let (input, blocks) = separated_list1(pair(newline, newline), block)(input)?;

    let blocks = blocks
        .into_iter()
        .map(|block| {
            let rows: Vec<String> = block.iter().map(|&s| s.to_string()).collect_vec();
            let cols: Vec<String> = (0..block[0].len())
                .map(|i| {
                    rows.iter()
                        .map(|row| row.chars().nth(i).unwrap().to_string())
                        .collect::<String>()
                })
                .collect_vec();

            (rows, cols)
        })
        .collect_vec();

    Ok((input, blocks))
}

#[instrument]
fn partition<T: Eq + Clone + std::fmt::Debug>(input: Vec<T>) -> (Vec<T>, Vec<T>) {
    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            let a = input[..=i].to_vec();
            let b = input[i + 1..].to_vec();
            if is_mirror(&a, &b) {
                return (a, b);
            }
        }
    }
    (input, vec![])
}

#[instrument]
fn is_mirror<T: PartialEq + Clone + std::fmt::Debug>(a: &[T], b: &[T]) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }

    let a: Vec<T> = a.iter().rev().cloned().collect();
    let b: Vec<T> = b.to_vec();
    if a.len() >= b.len() {
        return a.starts_with(&b);
    } else {
        return b.starts_with(&a);
    }
}

#[instrument]
fn smaller_len<T: std::fmt::Debug>(a: &Vec<T>, b: &Vec<T>) -> usize {
    a.len().min(b.len())
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, parsed) = parse(input).unwrap();

    debug_assert!(input.is_empty());

    Ok(parsed
        .iter()
        .map(|block| {
            let rows = block.0.clone();
            let cols = block.1.clone();

            let (row_lhs, row_rhs) = partition(rows);
            let (col_lhs, col_rhs) = partition(cols);

            let mut sum: usize = 0;
            if is_mirror(&col_lhs, &col_rhs) {
                sum += col_lhs.len();
            }
            if is_mirror(&row_lhs, &row_rhs) {
                sum += 100 * row_lhs.len();
            }

            sum
        })
        .sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test_log::test]
    fn tets_partition() {
        let a = vec!['a', 'b', 'c', 'd', 'd', 'c', 'b', 'a'];
        assert_eq!(
            partition(a),
            (vec!['a', 'b', 'c', 'd'], vec!['d', 'c', 'b', 'a'])
        );
    }

    #[test_log::test]
    fn tets_partition_double_col() {
        let a = vec!['a', 'b', 'b', 'c', 'd', 'd', 'c', 'b', 'b'];
        assert_eq!(
            partition(a),
            (vec!['a', 'b', 'b', 'c', 'd'], vec!['d', 'c', 'b', 'b'])
        );
    }

    #[test_log::test]
    fn test_is_mirror() {
        let a = vec!['a', 'b', 'c', 'd'];
        let b = vec!['d', 'c', 'b'];
        assert!(is_mirror(&a, &b));
    }

    #[test_log::test]
    fn test_is_mirror_empty() {
        let a = vec!['a', 'b', 'c', 'd'];
        let b = vec![];
        assert!(!is_mirror(&a, &b));
    }

    #[test_log::test]
    fn test_smaller_len() {
        let a = vec!['a', 'b', 'c', 'd'];
        let b = vec!['d', 'c', 'b'];
        assert_eq!(smaller_len(&a, &b), 3);
        assert_eq!(smaller_len(&b, &a), 3);
    }

    #[test_log::test(rstest)]
    #[case("test-input.txt", 405)]
    #[case("test-input2.txt", 408)]
    fn test_process(#[case] filename: &str, #[case] output: u64) -> miette::Result<()> {
        let input =
            String::from_utf8_lossy(&std::fs::read(std::path::Path::new(filename)).unwrap())
                .parse::<String>()
                .unwrap();
        assert_eq!(output, process(&input)?);
        Ok(())
    }
}
