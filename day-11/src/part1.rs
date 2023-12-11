use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

use crate::error::AocError;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Galax,
    Empty,
}

fn parse(input: &str) -> BTreeMap<Point, Tile> {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Galax,
                    '.' => Tile::Empty,
                    _ => panic!("Unknown tile type"),
                })
                .collect_vec()
        })
        .collect::<Vec<Vec<Tile>>>();

    let mut expanded_rows = Vec::new();
    for line in input.iter() {
        if line.iter().all(|t| matches!(t, Tile::Empty)) {
            expanded_rows.push(line.clone());
        }
        expanded_rows.push(line.clone());
    }

    let mut translated = Vec::new();
    for col_index in 0..expanded_rows[0].len() {
        let col = expanded_rows
            .iter()
            .map(|r| r[col_index].clone())
            .collect::<Vec<Tile>>();

        if col.iter().all(|t| matches!(t, Tile::Empty)) {
            translated.push(col.clone());
        }
        translated.push(col.clone());
    }

    translated
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter().enumerate().map(move |(x, tile)| {
                // Working off translated map, so re-translate back
                let pos = Point { x, y };
                (pos, tile.clone())
            })
        })
        .collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let map = parse(input);
    let locations = map
        .iter()
        .filter_map(|(p, t)| matches!(t, Tile::Galax).then_some(p.clone()))
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
        .iter()
        .map(|(p1, p2)| {
            let x = (p1.x as i64 - p2.x as i64).abs();
            let y = (p1.y as i64 - p2.y as i64).abs();
            (x + y) as usize
        })
        .collect();

    Ok(distances.iter().map(|&d| d as u64).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(374, process(input)?);
        Ok(())
    }
}
