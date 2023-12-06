use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
enum Value {
    Digit(u32),
    Symbol(char),
}

#[derive(Debug, Clone, Default, PartialOrd, Ord, Eq, PartialEq)]
struct Address {
    row: i32,
    col: i32,
}

fn part_1(input: &str) -> u32 {
    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            extract_numbers(line)
                .into_iter()
                .map(move |(col, len, number)| {
                    (
                        (col..col + len)
                            .map(|c| Address {
                                row: row as i32,
                                col: c as i32,
                            })
                            .collect::<Vec<_>>(),
                        number,
                    )
                })
        })
        .collect::<Vec<(Vec<Address>, u32)>>();

    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, character)| {
                (
                    Address {
                        row: row as i32,
                        col: col as i32,
                    },
                    match character {
                        '.' => None,
                        c if c.is_ascii_digit() => Some(Value::Digit(c.to_digit(10).unwrap())),
                        c => Some(Value::Symbol(c)),
                    },
                )
            })
        })
        .collect::<BTreeMap<Address, Option<Value>>>();

    let symbols: BTreeMap<Address, char> = grid
        .iter()
        .filter_map(|(address, value)| match value {
            Some(Value::Symbol(c)) => Some((address.clone(), *c)),
            _ => None,
        })
        .collect();

    let mut sum = 0;
    for (addresses, number) in numbers {
        if addresses
            .iter()
            .flat_map(bounding_box)
            .any(|a| symbols.contains_key(&a))
        {
            sum += number;
        }
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            extract_numbers(line)
                .into_iter()
                .map(move |(col, len, number)| {
                    (
                        (col..col + len)
                            .map(|c| Address {
                                row: row as i32,
                                col: c as i32,
                            })
                            .collect::<Vec<_>>(),
                        number,
                    )
                })
        })
        .collect::<Vec<(Vec<Address>, u32)>>();

    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, character)| {
                (
                    Address {
                        row: row as i32,
                        col: col as i32,
                    },
                    match character {
                        '.' => None,
                        c if c.is_ascii_digit() => Some(Value::Digit(c.to_digit(10).unwrap())),
                        c => Some(Value::Symbol(c)),
                    },
                )
            })
        })
        .collect::<BTreeMap<Address, Option<Value>>>();

    let gears: BTreeSet<Address> = grid
        .iter()
        .filter_map(|(address, value)| match value {
            Some(Value::Symbol('*')) => Some(address.clone()),
            _ => None,
        })
        .collect();

    dbg!(&gears);

    let mut sum = 0;
    for gear in gears {
        let bounding_numbers = numbers
            .iter()
            .filter_map(|(addresses, number)| {
                if addresses.iter().flat_map(bounding_box).any(|a| a == gear) {
                    Some(number)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if bounding_numbers.len() == 2 {
            println!("Including numbers {:?}", bounding_numbers);
            sum += bounding_numbers.into_iter().product::<u32>()
        }
    }

    sum
}

fn extract_numbers(input: &str) -> Vec<(usize, usize, u32)> {
    let mut result = Vec::default();
    let mut current_number = String::new();
    let mut current_offset = 0;

    for (offset, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            current_number.push(c);
            current_offset = offset + 1;
        } else if !current_number.is_empty() {
            result.push((
                current_offset - current_number.len(),
                current_number.len(),
                current_number.parse::<u32>().unwrap(),
            ));
            current_number.clear();
        }
    }

    if !current_number.is_empty() {
        result.push((
            current_offset - current_number.len(),
            current_number.len(),
            current_number.parse::<u32>().unwrap(),
        ));
    }

    result
}

fn bounding_box(address: &Address) -> Vec<Address> {
    vec![
        Address {
            row: address.row - 1,
            col: address.col - 1,
        },
        Address {
            row: address.row - 1,
            col: address.col,
        },
        Address {
            row: address.row - 1,
            col: address.col + 1,
        },
        Address {
            row: address.row,
            col: address.col - 1,
        },
        Address {
            row: address.row,
            col: address.col + 1,
        },
        Address {
            row: address.row + 1,
            col: address.col - 1,
        },
        Address {
            row: address.row + 1,
            col: address.col,
        },
        Address {
            row: address.row + 1,
            col: address.col + 1,
        },
    ]
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_extract_number() {
        assert_eq!(
            extract_numbers("123..45.6"),
            vec![(0, 3, 123), (5, 2, 45), (8, 1, 6)]
        );
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test-input.txt");
        assert_eq!(part_1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test-input.txt");
        assert_eq!(part_2(input), 467835);
    }
}
