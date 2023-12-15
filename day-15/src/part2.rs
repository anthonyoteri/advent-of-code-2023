use itertools::Itertools;

use crate::error::AocError;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1},
    combinator::opt,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct Instruction<'a> {
    hash: usize,
    label: &'a str,
    operation: Operation,
}

#[derive(Debug, Clone)]
enum Operation {
    Insert(u8),
    Remove,
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    power: u8,
}

#[derive(Debug)]
struct LightBox<'a> {
    lenses: Vec<Lens<'a>>,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, label) = alpha1(input)?;
    let (input, _) = is_a("=-")(input)?;
    let (input, power) = opt(complete::u8)(input)?;

    let operation = {
        match power {
            Some(p) => Operation::Insert(p),
            None => Operation::Remove,
        }
    };
    let instruction = Instruction {
        hash: hash_value(label),
        label,
        operation,
    };

    Ok((input, instruction))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag(","), parse_instruction)(input)
}

#[tracing::instrument]
fn hash_value(input: &str) -> usize {
    let ascii_codes = input.chars().map(|c| c as u8).collect_vec();

    let mut sum: usize = 0;

    for code in ascii_codes {
        sum += code as usize;
        sum *= 17;
        sum %= 256;
    }

    sum
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, instructions) = parse(input).unwrap();
    debug_assert!(input.is_empty(), "Failed to completely parse input");

    let boxes = (0..u8::MAX)
        .map(|_| LightBox { lenses: Vec::new() })
        .collect_vec();

    let boxes = instructions.iter().fold(boxes, |mut boxes, instruction| {
        match instruction.operation {
            Operation::Insert(power) => {
                let idx = boxes[instruction.hash]
                    .lenses
                    .iter()
                    .position(|l| l.label == instruction.label);
                match idx {
                    Some(idx) => {
                        boxes[instruction.hash].lenses[idx].power = power;
                    }
                    None => {
                        let lens = Lens {
                            label: instruction.label,
                            power,
                        };
                        boxes[instruction.hash].lenses.push(lens);
                    }
                }
            }
            Operation::Remove => boxes[instruction.hash]
                .lenses
                .retain(|l| l.label != instruction.label),
        };

        boxes
    });

    let power: u64 = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_index, b)| {
            b.lenses.iter().enumerate().map(move |(lens_index, l)| {
                let box_part = box_index + 1;
                let slot_part = lens_index + 1;
                let focal_length = l.power as usize;

                box_part as u64 * slot_part as u64 * focal_length as u64
            })
        })
        .sum();

    Ok(power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(145, process(input)?);
        Ok(())
    }
}
