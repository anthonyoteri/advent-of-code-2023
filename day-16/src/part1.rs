use crate::error::AocError;
use glam::IVec2;
use itertools::Itertools;
use nom::bytes::complete::is_a;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    RightMirror,
    LeftMirror,
    HorizontalSplit,
    VerticalSplit,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Beam {
    direction: Direction,
    position: IVec2,
}

fn parse(input: &str) -> IResult<&str, HashMap<IVec2, Tile>> {
    let (input, rows) = separated_list1(line_ending, is_a(r".|\/-"))(input)?;

    let grid = rows
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().map(move |(x, tile)| {
                let position = IVec2::new(x as i32, y as i32);
                let tile = match tile {
                    '.' => Tile::Empty,
                    '|' => Tile::VerticalSplit,
                    '/' => Tile::RightMirror,
                    '\\' => Tile::LeftMirror,
                    '-' => Tile::HorizontalSplit,
                    _ => unreachable!(),
                };
                (position, tile)
            })
        })
        .collect::<HashMap<IVec2, Tile>>();

    Ok((input, grid))
}

fn step(grid: &HashMap<IVec2, Tile>, beams: &Vec<Beam>) -> Vec<Beam> {
    beams
        .iter()
        .flat_map(|beam| {
            let position = match beam.direction {
                Direction::North => beam.position + IVec2::new(0, -1),
                Direction::South => beam.position + IVec2::new(0, 1),
                Direction::East => beam.position + IVec2::new(1, 0),
                Direction::West => beam.position + IVec2::new(-1, 0),
            };

            match grid.get(&position) {
                Some(Tile::VerticalSplit) => match beam.direction {
                    Direction::North | Direction::South => vec![Beam {
                        direction: beam.direction.clone(),
                        position,
                    }],
                    Direction::East | Direction::West => vec![
                        Beam {
                            direction: Direction::North,
                            position,
                        },
                        Beam {
                            direction: Direction::South,
                            position,
                        },
                    ],
                },

                Some(Tile::HorizontalSplit) => match beam.direction {
                    Direction::East | Direction::West => vec![Beam {
                        direction: beam.direction.clone(),
                        position,
                    }],
                    Direction::North | Direction::South => vec![
                        Beam {
                            direction: Direction::East,
                            position,
                        },
                        Beam {
                            direction: Direction::West,
                            position,
                        },
                    ],
                },
                Some(Tile::RightMirror) => match beam.direction {
                    Direction::North => vec![Beam {
                        direction: Direction::East,
                        position,
                    }],
                    Direction::South => vec![Beam {
                        direction: Direction::West,
                        position,
                    }],
                    Direction::East => vec![Beam {
                        direction: Direction::North,
                        position,
                    }],
                    Direction::West => vec![Beam {
                        direction: Direction::South,
                        position,
                    }],
                },
                Some(Tile::LeftMirror) => match beam.direction {
                    Direction::North => vec![Beam {
                        direction: Direction::West,
                        position,
                    }],
                    Direction::South => vec![Beam {
                        direction: Direction::East,
                        position,
                    }],
                    Direction::East => vec![Beam {
                        direction: Direction::South,
                        position,
                    }],
                    Direction::West => vec![Beam {
                        direction: Direction::North,
                        position,
                    }],
                },
                Some(Tile::Empty) | None => vec![Beam {
                    direction: beam.direction.clone(),
                    position,
                }],
            }
            .to_vec()
        })
        .collect_vec()
}

fn check_bounds(_grid: &HashMap<IVec2, Tile>, beams: &Vec<Beam>, boundary: &IVec2) -> Vec<Beam> {
    beams
        .iter()
        .filter(|beam| {
            beam.position.x <= boundary.x
                && beam.position.y <= boundary.y
                && beam.position.x >= 0
                && beam.position.y >= 0
        })
        .cloned()
        .collect_vec()
}

fn check_history(beams: &Vec<Beam>, visited: &HashSet<Beam>) -> Vec<Beam> {
    beams
        .iter()
        .filter(|beam| !visited.contains(beam))
        .cloned()
        .collect_vec()
}

fn visualize(grid: &HashMap<IVec2, Tile>, visited: &HashSet<Beam>) {
    let boundary = grid.keys().fold(IVec2::new(0, 0), |max, position| {
        IVec2::new(max.x.max(position.x), max.y.max(position.y))
    });

    for y in 0..=boundary.y {
        for x in 0..=boundary.x {
            let position = IVec2::new(x, y);
            if visited.contains(&Beam {
                direction: Direction::North,
                position,
            }) {
                print!("^");
            } else if visited.contains(&Beam {
                direction: Direction::South,
                position,
            }) {
                print!("v");
            } else if visited.contains(&Beam {
                direction: Direction::East,
                position,
            }) {
                print!(">");
            } else if visited.contains(&Beam {
                direction: Direction::West,
                position,
            }) {
                print!("<");
            } else if let Some(tile) = grid.get(&position) {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::VerticalSplit => print!("|"),
                    Tile::HorizontalSplit => print!("-"),
                    Tile::RightMirror => print!("/"),
                    Tile::LeftMirror => print!("\\"),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, grid) = parse(input).unwrap();
    debug_assert!(input.is_empty(), "Not all input was parsed");

    let boundary = grid.keys().fold(IVec2::new(0, 0), |max, position| {
        IVec2::new(max.x.max(position.x), max.y.max(position.y))
    });

    let mut visited: HashSet<Beam> = HashSet::new();

    let mut beams: Vec<Beam> = vec![Beam {
        direction: Direction::East,
        position: IVec2::new(-1, 0),
    }];
    //visited.extend(beams.iter().cloned());

    loop {
        beams = step(&grid, &beams);
        beams = check_bounds(&grid, &beams, &boundary);
        beams = check_history(&beams, &visited);
        visited.extend(beams.iter().cloned());

        if beams.is_empty() {
            break;
        }
    }

    #[cfg(test)]
    visualize(&grid, &visited);

    let energized = visited
        .iter()
        .map(|node| node.position)
        .collect::<HashSet<IVec2>>();

    Ok(energized.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
