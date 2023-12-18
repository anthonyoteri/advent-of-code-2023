use std::collections::{HashMap};

use crate::error::AocError;
use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{self, digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Default)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    distance: u32,
    color: Color,
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()), |v| {
        u8::from_str_radix(v, 16)
    })(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn parse_line(input: &str) -> IResult<&str, (Direction, u32, Color)> {
    let (input, direction) = alt((
        map(complete::char('R'), |_| Direction::East),
        map(complete::char('L'), |_| Direction::West),
        map(complete::char('U'), |_| Direction::North),
        map(complete::char('D'), |_| Direction::South),
    ))(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, color) = hex_color(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (direction, distance, color)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;

    Ok((
        input,
        lines
            .iter()
            .map(|(direction, distance, color)| Instruction {
                direction: direction.clone(),
                distance: *distance,
                color: color.clone(),
            })
            .collect(),
    ))
}

fn normalize(grid: &HashMap<IVec2, Color>) -> HashMap<IVec2, Color> {
    let min_x = grid.keys().map(|v| v.x).min().unwrap();
    let min_y = grid.keys().map(|v| v.y).min().unwrap();

    let origin = IVec2::new(min_x, min_y);

    let mut normalized = HashMap::new();
    for (pos, color) in grid.iter() {
        normalized.insert(*pos + origin.abs(), color.clone());
    }

    normalized
}

#[cfg(test)]
fn visualize(grid: &HashMap<IVec2, Color>) {
    let min_x = grid.keys().map(|v| v.x).min().unwrap();
    let min_y = grid.keys().map(|v| v.y).min().unwrap();
    let max_x = grid.keys().map(|v| v.x).max().unwrap();
    let max_y = grid.keys().map(|v| v.y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let pos = IVec2::new(x, y);
            if let Some(color) = grid.get(&pos) {
                print!(
                    "\x1b[48;2;{};{};{}m  \x1b[0m",
                    color.red, color.green, color.blue
                );
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn flood_fill(grid: &HashMap<IVec2, Color>) -> HashMap<IVec2, Color> {
    let min_x = grid.keys().map(|v| v.x).min().unwrap();
    let min_y = grid.keys().map(|v| v.y).min().unwrap();
    let max_x = grid.keys().map(|v| v.x).max().unwrap();
    let max_y = grid.keys().map(|v| v.y).max().unwrap();

    let mut filled = grid.clone();
    for y in min_y..max_y {
        let mut is_inside = false;
        for x in min_x..=max_x {
            if grid.contains_key(&IVec2::new(x, y)) && grid.contains_key(&IVec2::new(x, y + 1)) {
                is_inside = !is_inside
            } else if is_inside {
                filled.insert(
                    IVec2::new(x, y),
                    Color {
                        red: 128,
                        green: 128,
                        blue: 128,
                    },
                );
            }
        }
    }
    filled
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, instructions) = parse(input).unwrap();

    debug_assert!(input.is_empty());

    let mut grid: HashMap<IVec2, Color> = HashMap::new();
    let mut current = IVec2::splat(0);
    instructions.into_iter().for_each(|instruction| {
        (0..instruction.distance).for_each(|_| {
            match instruction.direction {
                Direction::North => current.y += 1,
                Direction::South => current.y -= 1,
                Direction::East => current.x += 1,
                Direction::West => current.x -= 1,
            };
            grid.insert(current, instruction.color.clone());
        });
    });

    let normalized = normalize(&grid);
    let filled = flood_fill(&normalized);

    #[cfg(test)]
    visualize(&filled);

    Ok(filled.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(62, process(input)?);
        Ok(())
    }
}
