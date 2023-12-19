use std::collections::HashSet;

use crate::error::AocError;
use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while_m_n},
    character::complete::{self, digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    distance: u32,
}
fn parse_distance(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit()), |v| {
        u32::from_str_radix(v, 16)
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, (Direction, u32)> {
    let (input, _) = alt((
        map(complete::char('R'), |_| Direction::East),
        map(complete::char('L'), |_| Direction::West),
        map(complete::char('U'), |_| Direction::North),
        map(complete::char('D'), |_| Direction::South),
    ))(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(#")(input)?;
    let (input, distance) = parse_distance(input)?;
    let (input, direction) = alt((
        map(complete::char('0'), |_| Direction::East),
        map(complete::char('2'), |_| Direction::West),
        map(complete::char('3'), |_| Direction::North),
        map(complete::char('1'), |_| Direction::South),
    ))(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (direction, distance)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;

    Ok((
        input,
        lines
            .iter()
            .map(|(direction, distance)| Instruction {
                direction: direction.clone(),
                distance: *distance,
            })
            .collect(),
    ))
}

fn normalize(grid: &HashSet<IVec2>) -> HashSet<IVec2> {
    let min_x = grid.par_iter().map(|v| v.x).min().unwrap();
    let min_y = grid.par_iter().map(|v| v.y).min().unwrap();

    let origin = IVec2::new(min_x, min_y);

    let mut normalized = HashSet::new();
    for pos in grid.iter() {
        normalized.insert(*pos + origin.abs());
    }

    normalized
}

fn detect_edges(grid: &HashSet<IVec2>) -> HashSet<IVec2> {
    let min_x = grid.par_iter().map(|v| v.x).min().unwrap();
    let min_y = grid.par_iter().map(|v| v.y).min().unwrap();
    let max_x = grid.par_iter().map(|v| v.x).max().unwrap();
    let max_y = grid.par_iter().map(|v| v.y).max().unwrap();

    let mut result = grid.clone();
    for x in min_x..=max_x {
        let mut is_inside = false;
        for y in min_y..max_y {
            if grid.contains(&IVec2::new(x, y)) {
                is_inside = !is_inside
            } else if is_inside {
                result.insert(IVec2::new(x, y));
            }
        }
    }

    result
}

fn flood_fill(grid: &HashSet<IVec2>) -> u64 {
    let min_x = grid.iter().map(|v| v.x).min().unwrap();
    let min_y = grid.iter().map(|v| v.y).min().unwrap();
    let max_x = grid.iter().map(|v| v.x).max().unwrap();
    let max_y = grid.iter().map(|v| v.y).max().unwrap();

    let mut grid = detect_edges(&grid);

    let mut sum: u64 = 0;
    for y in min_y..max_y {
        let mut is_inside = false;
        for x in min_x..=max_x {
            if grid.contains(&IVec2::new(x, y)) && grid.contains(&IVec2::new(x, y + 1)) {
                is_inside = !is_inside
            } else if is_inside {
                sum += 1;
            }
        }
    }
    sum
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, instructions) = parse(input).unwrap();

    debug_assert!(input.is_empty());

    let mut grid: HashSet<IVec2> = HashSet::new();
    let mut current = IVec2::splat(0);
    instructions.into_iter().for_each(|instruction| {
        grid.insert(current);
        match instruction.direction {
            Direction::North => current.y += instruction.distance as i32,
            Direction::South => current.y -= instruction.distance as i32,
            Direction::East => current.x += instruction.distance as i32,
            Direction::West => current.x -= instruction.distance as i32,
        };
    });

    let normalized = normalize(&grid);
    let filled = flood_fill(&normalized);

    //#[cfg(test)]
    //visualize(&filled);

    Ok(normalized.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(952408144115, process(input)?);
        Ok(())
    }
}
