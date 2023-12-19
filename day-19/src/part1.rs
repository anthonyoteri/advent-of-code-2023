use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{map, opt},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser as _,
};

use crate::error::AocError;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Condition {
    LessThen,
    GreaterThen,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Target<'a> {
    Workflow(&'a str),
    Accept,
    Reject,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Rule<'a> {
    Test {
        field: Field,
        condition: Condition,
        value: u16,
        target: Target<'a>,
    },
    Target(Target<'a>),
}

impl<'a> Rule<'a> {
    fn evaluate(&self, part: &'a Part) -> Option<Target<'a>> {
        match self {
            Rule::Test {
                field,
                condition,
                value,
                target,
            } => {
                let field_value = match field {
                    Field::X => part.x,
                    Field::M => part.m,
                    Field::A => part.a,
                    Field::S => part.s,
                };
                match condition {
                    Condition::LessThen => {
                        if field_value < *value {
                            Some(target.clone())
                        } else {
                            None
                        }
                    }
                    Condition::GreaterThen => {
                        if field_value > *value {
                            Some(target.clone())
                        } else {
                            None
                        }
                    }
                }
            }
            Rule::Target(target) => Some(target.clone()),
        }
    }
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, field) = complete::alpha1(input)?;
    let (input, condition) = alt((
        complete::char('<').map(|_| Condition::LessThen),
        complete::char('>').map(|_| Condition::GreaterThen),
    ))(input)?;
    let (input, value) = complete::u16(input)?;
    let (input, target) = preceded(complete::char(':'), target)(input)?;

    Ok((
        input,
        Rule::Test {
            field: match field {
                "x" => Field::X,
                "m" => Field::M,
                "a" => Field::A,
                "s" => Field::S,
                _ => unreachable!(),
            },
            condition,
            value,
            target,
        },
    ))
}

fn target(input: &str) -> IResult<&str, Target> {
    alt((
        map(tag("A"), |_| Target::Accept),
        map(tag("R"), |_| Target::Reject),
        complete::alpha1.map(Target::Workflow),
    ))(input)
}

fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, id) = complete::alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(complete::char(','), alt((rule, target.map(Rule::Target)))),
        complete::char('}'),
    )(input)?;
    Ok((input, Workflow { id, rules }))
}

fn workflows(input: &str) -> IResult<&str, BTreeMap<&str, Workflow>> {
    let (input, workflows) = separated_list1(complete::line_ending, workflow)(input)?;
    Ok((input, workflows.into_iter().map(|w| (w.id, w)).collect()))
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

fn part(input: &str) -> IResult<&str, Part> {
    delimited(
        complete::char('{'),
        fold_many1(
            terminated(
                separated_pair(complete::alpha1, complete::char('='), complete::u16),
                opt(complete::char(',')),
            ),
            Part::default,
            |mut part, (field, count)| {
                match field {
                    "x" => part.x = count,
                    "m" => part.m = count,
                    "a" => part.a = count,
                    "s" => part.s = count,
                    _ => unreachable!(),
                }
                part
            },
        ),
        complete::char('}'),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (BTreeMap<&str, Workflow>, Vec<Part>)> {
    let (input, workflows) = workflows(input)?;
    let (input, _) = complete::multispace1(input)?;
    let (input, parts) = separated_list1(complete::line_ending, part)(input)?;

    Ok((input, (workflows, parts)))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, (workflows, parts)) = parse(input).unwrap();
    debug_assert!(input.is_empty(), "Should have parsed all the input");

    let results = parts
        .iter()
        .filter_map(|part| {
            let mut current = "in";
            let target = 'outer: loop {
                let active = workflows.get(current).unwrap();
                'inner: for rule in &active.rules {
                    match rule.evaluate(part) {
                        Some(Target::Accept) => break 'outer Target::Accept,
                        Some(Target::Reject) => break 'outer Target::Reject,
                        Some(Target::Workflow(id)) => {
                            current = id;
                            break 'inner;
                        }
                        None => {}
                    }
                }
            };
            match target {
                Target::Accept => {
                    Some(part.x as u64 + part.m as u64 + part.a as u64 + part.s as u64)
                }
                Target::Reject => None,
                _ => unreachable!(),
            }
        })
        .sum::<u64>();

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test-input.txt");
        assert_eq!(19114, process(input)?);
        Ok(())
    }
}
