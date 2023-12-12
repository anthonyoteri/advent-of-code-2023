use std::collections::BTreeSet;

use itertools::Itertools;

use crate::error::AocError;
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> BTreeSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point { x, y }),
                '.' => None,
                _ => panic!("Unknown tile type"),
            })
        })
        .collect::<BTreeSet<Point>>()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let map = parse(input);

    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for n in 0..map.len() {
        let empty_row: bool = !map.iter().any(|k| k.y == n);
        let empty_col: bool = !map.iter().any(|k| k.x == n);
        if empty_row {
            empty_rows.push(n);
        }
        if empty_col {
            empty_cols.push(n);
        }
    }

    let locations = map
        .iter()
        .map(|m| {
            let dx = empty_cols.iter().filter(|&c| c < &m.x).count();
            let dy = empty_rows.iter().filter(|&r| r < &m.y).count();

            Point {
                x: m.x - dx + 1_000_000 * dx,
                y: m.y - dy + 1_000_000 * dy,
            }
        })
        .collect::<BTreeSet<Point>>();

    let pairs: BTreeSet<BTreeSet<Point>> = locations
        .iter()
        .cartesian_product(locations.iter())
        .filter(|(p1, p2)| p1 != p2)
        .map(|(p1, p2)| BTreeSet::from([p1.clone(), p2.clone()]))
        .collect();

    let pairs: Vec<(Point, Point)> = pairs
        .into_iter()
        .map(|pair| {
            let mut pair = pair.into_iter();
            let p1 = pair.next().unwrap();
            let p2 = pair.next().unwrap();
            (p1, p2)
        })
        .collect();
    let distances: Vec<usize> = pairs
        .par_iter()
        .map(|(p1, p2)| {
            let x = (p1.x as i64 - p2.x as i64).abs();
            let y = (p1.y as i64 - p2.y as i64).abs();
            (x + y) as usize
        })
        .collect();

    Ok(distances.par_iter().map(|&d| d as u64).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(82000210, process(input)?);
        Ok(())
    }
}
