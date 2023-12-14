use crate::error::AocError;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult,
};
use rayon::prelude::*;
use std::cmp::Ordering;

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        line_ending,
        many1(alt((
            complete::char('#'),
            complete::char('.'),
            complete::char('O'),
        ))),
    )(input)
}

fn partial_cmp(a: &char, b: &char) -> Option<Ordering> {
    match (a, b) {
        /*         ('#', '#') => Some(Ordering::Equal),
        ('#', '.') => Some(Ordering::Equal),
        ('#', 'O') => Some(Ordering::Equal),
        ('.', '#') => Some(Ordering::Equal),
        ('.', '.') => Some(Ordering::Equal),
        ('.', 'O') => Some(Ordering::Greater),
        ('O', '#') => Some(Ordering::Equal),
        ('O', '.') => Some(Ordering::Less),
        ('O', 'O') => Some(Ordering::Equal),
        _ => None, */
        ('O', 'O') => Some(Ordering::Equal),
        ('O', _) => Some(Ordering::Less),
        (_, 'O') => Some(Ordering::Greater),
        (_, _) => Some(Ordering::Equal),
    }
}

fn custom_sort(input: &[char]) -> Vec<char> {
    input
        .split_inclusive(|&c| c == '#')
        .map(|chunk| {
            let mut chunk = chunk.to_vec();
            chunk.sort_by(|a, b| partial_cmp(a, b).unwrap());
            chunk
        })
        .collect_vec()
        .concat()
}

#[memoize::memoize]
fn transpose(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut columns_iter_collection = input.iter().map(|line| line.iter()).collect::<Vec<_>>();

    std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => {
                    items.push(*item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>()
}

#[memoize::memoize]
pub fn sort_up(input: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    let mut columns = transpose(input);
    columns = columns
        .par_iter()
        .map(|c| custom_sort(c))
        .collect::<Vec<_>>();
    let rows = transpose(columns);

    Some(rows)
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, grid) = parse(input).unwrap();
    debug_assert!(input.is_empty());

    let sorted_up = sort_up(grid);
    let sum: u64 = sorted_up
        .iter()
        .flat_map(|row| {
            row.iter().rev().enumerate().map(move |(i, col)| {
                tracing::info!("{}: {}", i + 1, col.iter().filter(|&c| *c == 'O').count());
                col.iter().filter(|&c| *c == 'O').count() as u64 * (i + 1) as u64
            })
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case("O....#....", "O....#....")] // 1
    #[case("O....#....", "O....#....")] // 2
    #[case(".....##...", ".....##...")] // 3
    #[case("OO.#O....O", "OO.#OO....")] // 4
    #[case(".O.....O#.", "OO......#.")] // 5
    #[case("O.#..O.#.#", "O.#O...#.#")] // 6
    #[case("..O..#O..O", "O....#OO..")] // 7
    #[case(".......O..", "O.........")] // 8
    #[case("#....###..", "#....###..")] // 9
    #[case("#OO..#....", "#OO..#....")] // 10
    fn test_sorting(#[case] input: &str, #[case] output: &str) {
        let chars = input.chars().collect_vec();
        let sorted = custom_sort(&chars);

        assert_eq!(sorted, output.chars().collect_vec());
    }

    #[test_log::test]
    fn test_sort_up() {
        let input = include_str!("../test-input.txt");
        let (input, grid) = parse(input).unwrap();
        debug_assert!(input.is_empty());

        let sorted_up = sort_up(grid);

        assert_eq!(
            sorted_up.unwrap(),
            vec![
                vec!['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
                vec!['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
                vec!['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
                vec!['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
                vec!['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
                vec!['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            ]
        );
    }

    #[test_log::test(rstest)]
    #[case("test-input.txt", 136)]
    #[case("test-input2.txt", 108)]
    #[case("test-input3.txt", 136)]
    #[case("test-input4.txt", 106648)]
    fn test_process(#[case] filename: &str, #[case] expected: u64) -> miette::Result<()> {
        let input =
            String::from_utf8_lossy(&std::fs::read(std::path::Path::new(filename)).unwrap())
                .parse::<String>()
                .unwrap();
        assert_eq!(expected, process(&input)?);
        Ok(())
    }
}
