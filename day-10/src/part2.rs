use crate::error::AocError;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Pipe {
    input: Point,
    output: Point,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tile {
    Vertical(Pipe),
    Horizontal(Pipe),
    NorthEast(Pipe),
    NorthWest(Pipe),
    SouthWest(Pipe),
    SouthEast(Pipe),
    Ground,
    Start,
}

fn parse(input: &str) -> BTreeMap<Point, Tile> {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, c)| {
                let pos = Point {
                    row: row as i32,
                    col: col as i32,
                };

                let tile = match c {
                    '|' => Tile::Vertical(Pipe {
                        input: Point {
                            row: pos.row - 1,
                            col: pos.col,
                        },
                        output: Point {
                            row: pos.row + 1,
                            col: pos.col,
                        },
                    }),
                    '-' => Tile::Horizontal(Pipe {
                        input: Point {
                            row: pos.row,
                            col: pos.col - 1,
                        },
                        output: Point {
                            row: pos.row,
                            col: pos.col + 1,
                        },
                    }),
                    'L' => Tile::NorthEast(Pipe {
                        input: Point {
                            row: pos.row - 1,
                            col: pos.col,
                        },
                        output: Point {
                            row: pos.row,
                            col: pos.col + 1,
                        },
                    }),
                    'J' => Tile::NorthWest(Pipe {
                        input: Point {
                            row: pos.row - 1,
                            col: pos.col,
                        },
                        output: Point {
                            row: pos.row,
                            col: pos.col - 1,
                        },
                    }),
                    '7' => Tile::SouthWest(Pipe {
                        input: Point {
                            row: pos.row + 1,
                            col: pos.col,
                        },
                        output: Point {
                            row: pos.row,
                            col: pos.col - 1,
                        },
                    }),
                    'F' => Tile::SouthEast(Pipe {
                        input: Point {
                            row: pos.row + 1,
                            col: pos.col,
                        },
                        output: Point {
                            row: pos.row,
                            col: pos.col + 1,
                        },
                    }),
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => panic!("Unknown tile: {}", c),
                };

                (pos, tile)
            })
        })
        .collect::<BTreeMap<Point, Tile>>();

    grid
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let grid = parse(input);

    let mut main_loop = BTreeMap::default();

    let start: &Point = grid
        .iter()
        .find(|(_, tile)| **tile == Tile::Start)
        .map(|(pos, _)| pos)
        .unwrap();

    let start_connects: Vec<&Point> = grid
        .iter()
        .filter(|(_, tile)| match tile {
            Tile::Vertical(pipe) => pipe.input == *start || pipe.output == *start,
            Tile::Horizontal(pipe) => pipe.input == *start || pipe.output == *start,
            Tile::NorthEast(pipe) => pipe.input == *start || pipe.output == *start,
            Tile::NorthWest(pipe) => pipe.input == *start || pipe.output == *start,
            Tile::SouthWest(pipe) => pipe.input == *start || pipe.output == *start,
            Tile::SouthEast(pipe) => pipe.input == *start || pipe.output == *start,
            _ => false,
        })
        .map(|(pos, _)| pos)
        .collect();

    let mut current = *start_connects.first().unwrap();
    let mut prev = start;
    while current != start {
        let tile = grid.get(current).unwrap();
        match tile {
            Tile::Vertical(p)
            | Tile::Horizontal(p)
            | Tile::NorthEast(p)
            | Tile::NorthWest(p)
            | Tile::SouthEast(p)
            | Tile::SouthWest(p) => {
                if p.input == *prev {
                    prev = current;
                    current = &p.output;
                    main_loop.insert(current, tile);
                    main_loop.insert(prev, tile);
                } else {
                    prev = current;
                    current = &p.input;
                    main_loop.insert(current, tile);
                    main_loop.insert(prev, tile);
                }
            }
            _ => panic!("Unknown tile: {:?}", tile),
        }
    }

    Ok(grid
        .iter()
        .filter(|(pos, _)| !main_loop.contains_key(pos))
        .filter_map(|(pos, _)| {
            let tiles_in_row = main_loop.keys().filter(|p| p.row == pos.row);
            let count_west = tiles_in_row
                .clone()
                .filter(|t| {
                    matches!(
                        main_loop.get(*t),
                        Some(Tile::Vertical(_))
                            | Some(Tile::Start)
                            | Some(Tile::SouthEast(_))
                            | Some(Tile::SouthWest(_))
                    ) && t.col < pos.col
                })
                .count();
            let count_east = tiles_in_row
                .clone()
                .filter(|t| {
                    matches!(
                        main_loop.get(*t),
                        Some(Tile::Vertical(_))
                            | Some(Tile::Start)
                            | Some(Tile::SouthEast(_))
                            | Some(Tile::SouthWest(_))
                    ) && t.col > pos.col
                })
                .count();

            if count_west % 2 != 0 && (count_east % 2 != 0 || count_east != 0) {
                Some(1)
            } else {
                None
            }
        })
        .count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case("test-input3.txt", 4)]
    #[case("test-input4.txt", 8)]
    #[case("test-input5.txt", 10)]
    fn test_process(#[case] filename: &str, #[case] expected: usize) -> miette::Result<()> {
        let input =
            String::from_utf8_lossy(&std::fs::read(std::path::Path::new(filename)).unwrap())
                .parse::<String>()
                .unwrap();
        assert_eq!(expected, process(&input)? as usize);
        Ok(())
    }
}
