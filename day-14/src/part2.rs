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
use std::{cmp::Ordering, collections::BTreeSet};

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
        ('O', 'O') => Some(Ordering::Equal),
        ('O', _) => Some(Ordering::Less),
        (_, 'O') => Some(Ordering::Greater),
        (_, _) => Some(Ordering::Equal),
    }
}
fn partial_rev_cmp(a: &char, b: &char) -> Option<Ordering> {
    match (a, b) {
        ('#', 'O') => Some(Ordering::Greater),
        ('O', 'O') => Some(Ordering::Equal),
        ('O', _) => Some(Ordering::Greater),
        (_, 'O') => Some(Ordering::Less),
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

fn custom_rev_sort(input: &[char]) -> Vec<char> {
    input
        .split_inclusive(|&c| c == '#')
        .map(|chunk| {
            let mut chunk = chunk.to_vec();
            chunk.sort_by(|a, b| partial_rev_cmp(a, b).unwrap());
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
pub fn sort_north(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut columns = transpose(input);
    columns = columns
        .par_iter()
        .map(|c| custom_sort(c))
        .collect::<Vec<_>>();

    transpose(columns)
}

#[memoize::memoize]
pub fn sort_south(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut columns = transpose(input);
    columns = columns
        .par_iter()
        .map(|c| custom_rev_sort(c))
        .collect::<Vec<_>>();

    transpose(columns)
}

#[memoize::memoize]
pub fn sort_west(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input.par_iter().map(|c| custom_sort(c)).collect::<Vec<_>>()
}

#[memoize::memoize]
pub fn sort_east(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input
        .par_iter()
        .map(|c| custom_rev_sort(c))
        .collect::<Vec<_>>()
}

#[memoize::memoize]
pub fn cycle(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north = sort_north(input);
    let west = sort_west(north);
    let south = sort_south(west);
    sort_east(south)
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

fn get_cycle_len(input: &[Vec<char>]) -> (usize, usize) {
    // Return lenght of the initial input and the cycle length.

    let mut seen: BTreeSet<Vec<Vec<char>>> = BTreeSet::default();

    let mut cycled = input.to_owned();
    loop {
        cycled = cycle(cycled);
        if seen.contains(&cycled) {
            break;
        }
        seen.insert(cycled.clone());
    }

    let initial_cycle_count = seen.len();

    // Repeat the logic 2x to eliminate the effect of the initial few
    // permutations that are not part of the cycle.
    seen.clear();

    loop {
        cycled = cycle(cycled);
        if seen.contains(&cycled) {
            break;
        }
        seen.insert(cycled.clone());
    }

    let cycle_count = initial_cycle_count - seen.len();

    (cycle_count, initial_cycle_count - cycle_count)
}

fn weight(input: &[Vec<char>]) -> u64 {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&c| *c == 'O').count() as u64 * (i + 1) as u64)
        .sum()
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, grid) = parse(input).unwrap();
    debug_assert!(input.is_empty());

    let (initial_perms, cycle_len) = get_cycle_len(&grid);

    let mut cycled = grid.clone();
    for _ in 0..initial_perms {
        cycled = cycle(cycled);
    }

    let n = 1000000000;
    let remainder = (n - initial_perms) % cycle_len;

    for _ in 0..remainder {
        cycled = cycle(cycled);
    }

    Ok(weight(&cycled))
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
    fn test_sorting_forward(#[case] input: &str, #[case] output: &str) {
        let chars = input.chars().collect_vec();
        let sorted = custom_sort(&chars);

        assert_eq!(sorted, output.chars().collect_vec());
    }

    #[test_log::test(rstest)]
    #[case("O...#....", "...O#....")] // 1
    #[case("O....#....", "....O#....")] // 2
    #[case(".....##...", ".....##...")] // 3
    #[case("OO.#O....O", ".OO#....OO")] // 4
    #[case(".O.....O#.", "......OO#.")] // 5
    #[case("O.#..O.#.#", ".O#...O#.#")] // 6
    #[case("..O..#O..O", "....O#..OO")] // 7
    #[case(".......O..", ".........O")] // 8
    #[case("#....###..", "#....###..")] // 9
    #[case("#OO..#....", "#..OO#....")] // 10
    fn test_sorting_reverse(#[case] input: &str, #[case] output: &str) {
        let chars = input.chars().collect_vec();
        let sorted = custom_rev_sort(&chars);

        assert_eq!(sorted, output.chars().collect_vec());
    }

    #[test_log::test]
    fn test_sort_north() {
        let input = include_str!("../test-input.txt");
        let (input, grid) = parse(input).unwrap();
        debug_assert!(input.is_empty());

        let north = sort_north(grid);

        let expected = [
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ];

        assert_eq!(
            north,
            expected
                .iter()
                .map(|s| s.chars().collect_vec())
                .collect_vec()
        );
    }

    #[test_log::test]
    fn test_sort_south() {
        let input = include_str!("../test-input.txt");
        let (input, grid) = parse(input).unwrap();
        debug_assert!(input.is_empty());

        let south = sort_south(grid);
        let expected = [
            ".....#....",
            "....#....#",
            "...O.##...",
            "...#......",
            "O.O....O#O",
            "O.#..O.#.#",
            "O....#....",
            "OO....OO..",
            "#OO..###..",
            "#OO.O#...O",
        ];

        assert_eq!(
            south,
            expected
                .iter()
                .map(|s| s.chars().collect_vec())
                .collect_vec()
        );
    }

    #[test_log::test]
    fn test_sort_east() {
        let input = include_str!("../test-input.txt");
        let (input, grid) = parse(input).unwrap();
        debug_assert!(input.is_empty());

        let east = sort_east(grid);

        let expected = [
            "....O#....",
            ".OOO#....#",
            ".....##...",
            ".OO#....OO",
            "......OO#.",
            ".O#...O#.#",
            "....O#..OO",
            ".........O",
            "#....###..",
            "#..OO#....",
        ];

        assert_eq!(
            east,
            expected
                .iter()
                .map(|s| s.chars().collect_vec())
                .collect_vec()
        );
    }

    #[test_log::test]
    fn test_sort_west() {
        let input = include_str!("../test-input.txt");
        let (input, grid) = parse(input).unwrap();
        debug_assert!(input.is_empty());

        let west = sort_west(grid);

        let expected = [
            "O....#....",
            "OOO.#....#",
            ".....##...",
            "OO.#OO....",
            "OO......#.",
            "O.#O...#.#",
            "O....#OO..",
            "O.........",
            "#....###..",
            "#OO..#....",
        ];
        assert_eq!(
            west,
            expected
                .iter()
                .map(|s| s.chars().collect_vec())
                .collect_vec()
        );
    }

    #[test_log::test]
    fn test_cycle() {
        let input = include_str!("../test-input.txt");
        let (input, grid) = parse(input).unwrap();
        debug_assert!(input.is_empty());

        let cycle = cycle(grid);

        let expected = [
            ".....#....",
            "....#...O#",
            "...OO##...",
            ".OO#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#....",
            "......OOOO",
            "#...O###..",
            "#..OO#....",
        ];

        assert_eq!(
            cycle,
            expected
                .iter()
                .map(|s| s.chars().collect_vec())
                .collect_vec()
        );
    }

    #[test_log::test(rstest)]
    #[case("test-input.txt", 64)]
    fn test_process(#[case] filename: &str, #[case] expected: u64) -> miette::Result<()> {
        let input =
            String::from_utf8_lossy(&std::fs::read(std::path::Path::new(filename)).unwrap())
                .parse::<String>()
                .unwrap();
        assert_eq!(expected, process(&input)?);
        Ok(())
    }
}
