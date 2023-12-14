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

#[instrument]
fn get_permutations(block: &(Vec<String>, Vec<String>)) -> Vec<(Vec<String>, Vec<String>)> {
    let rows = block.0.clone();
    let cols = block.1.clone();

    let mut permutations = vec![];

    for i in 0..rows.len() {
        for j in 0..cols.len() {
            let mut rows = rows.clone();
            let mut cols = cols.clone();

            if let Some(c) = rows[i].chars().nth(j) {
                let c = match c {
                    '#' => ".",
                    '.' => "#",
                    _ => panic!("unexpected character"),
                };

                rows[i].replace_range(j..=j, &c);
            }
            if let Some(c) = cols[j].chars().nth(i) {
                let c = match c {
                    '#' => ".",
                    '.' => "#",
                    _ => panic!("unexpected character"),
                };

                cols[j].replace_range(i..=i, &c);
            }
            permutations.push((rows, cols));
        }
    }
    permutations
}

#[instrument]
fn get_reflection(block: &(Vec<String>, Vec<String>)) -> Option<usize> {
    let rows = block.0.clone();
    let cols = block.1.clone();

    let (row_lhs, row_rhs) = partition(rows);
    let (col_lhs, col_rhs) = partition(cols);

    if is_mirror(&row_lhs, &row_rhs) {
        return Some(100 * row_lhs.len());
    }
    if is_mirror(&col_lhs, &col_rhs) {
        return Some(col_lhs.len());
    }

    None
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, parsed) = parse(input).unwrap();

    debug_assert!(input.is_empty());

    Ok(parsed
        .iter()
        .flat_map(|b| {
            let old_score = get_reflection(b).unwrap_or(0);
            let scores = get_permutations(b)
                .iter()
                .filter_map(get_reflection)
                .filter(|&s| s != old_score)
                .collect_vec();

            let new_score = *scores.iter().max().unwrap_or(&0);
            dbg!(&old_score, &new_score);

            if new_score != 0 {
                Some(new_score)
            } else {
                Some(old_score)
            }
        })
        .inspect(|s| {
            dbg!(s);
            ()
        })
        .sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test_log::test]
    fn test_permutations() {
        let input = (
            vec!["..".to_string(), "..".to_string()],
            vec!["..".to_string(), "..".to_string()],
        );

        let output = vec![
            (
                vec!["#.".to_string(), "..".to_string()],
                vec!["#.".to_string(), "..".to_string()],
            ),
            (
                vec![".#".to_string(), "..".to_string()],
                vec!["..".to_string(), "#.".to_string()],
            ),
            (
                vec!["..".to_string(), "#.".to_string()],
                vec![".#".to_string(), "..".to_string()],
            ),
            (
                vec!["..".to_string(), ".#".to_string()],
                vec!["..".to_string(), ".#".to_string()],
            ),
        ];

        assert_eq!(get_permutations(&input), output);
    }

    #[test_log::test(rstest)]
    #[case("test-input.txt", 400)]
    #[case("test-input3.txt", 1400)]
    fn test_process(#[case] filename: &str, #[case] output: u64) -> miette::Result<()> {
        let input =
            String::from_utf8_lossy(&std::fs::read(std::path::Path::new(filename)).unwrap())
                .parse::<String>()
                .unwrap();
        assert_eq!(output, process(&input)?);
        Ok(())
    }
}
