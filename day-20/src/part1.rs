use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use crate::error::AocError;

#[derive(Debug)]
enum State {
    On,
    Off,
}

#[derive(Debug, Copy, Clone)]
enum Signal {
    High,
    Low,
}

#[derive(Debug)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { state: State },
    Conjunction { inputs: HashMap<&'a str, Signal> },
    Test,
}

#[derive(Debug)]
struct Module<'a> {
    id: &'a str,
    module_type: ModuleType<'a>,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    #[tracing::instrument]
    fn process(&mut self, from: &'a str, signal: &Signal) -> Vec<(&'a str, &'a str, Signal)> {
        match (&mut self.module_type, signal) {
            (ModuleType::Broadcaster, signal) => self
                .outputs
                .iter()
                .map(|o| (self.id, *o, *signal))
                .collect(),
            (ModuleType::FlipFlop { .. }, Signal::High) => {
                vec![]
            }
            (ModuleType::FlipFlop { state: State::On }, Signal::Low) => {
                self.module_type = ModuleType::FlipFlop { state: State::Off };
                self.outputs
                    .iter()
                    .map(|o| (self.id, *o, Signal::Low))
                    .collect()
            }
            (ModuleType::FlipFlop { state: State::Off }, Signal::Low) => {
                self.module_type = ModuleType::FlipFlop { state: State::On };
                self.outputs
                    .iter()
                    .map(|o| (self.id, *o, Signal::High))
                    .collect()
            }
            (ModuleType::Conjunction { ref mut inputs }, pulse) => {
                inputs.entry(from).and_modify(|s| *s = *pulse);
                if inputs.values().all(|s| matches!(s, Signal::High)) {
                    self.outputs
                        .iter()
                        .map(|o| (self.id, *o, Signal::Low))
                        .collect()
                } else {
                    self.outputs
                        .iter()
                        .map(|o| (self.id, *o, Signal::High))
                        .collect()
                }
            }
            _ => {
                vec![]
            }
        }
    }
}

fn broadcaster(input: &str) -> IResult<&str, Module> {
    let (input, id) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((
        input,
        Module {
            id,
            module_type: ModuleType::Broadcaster,
            outputs,
        },
    ))
}

fn flip_flop(input: &str) -> IResult<&str, Module> {
    let (input, id) = preceded(nom::character::complete::char('%'), alpha1)(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((
        input,
        Module {
            id,
            module_type: ModuleType::FlipFlop { state: State::Off },
            outputs,
        },
    ))
}

fn conjunction(input: &str) -> IResult<&str, Module> {
    let (input, id) = preceded(nom::character::complete::char('&'), alpha1)(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((
        input,
        Module {
            id,
            module_type: ModuleType::Conjunction {
                inputs: HashMap::new(),
            },
            outputs,
        },
    ))
}

fn parser(input: &str) -> IResult<&str, HashMap<&str, Module>> {
    let (input, modules) =
        separated_list1(line_ending, alt((broadcaster, flip_flop, conjunction)))(input)?;

    let mut module_map = modules
        .into_iter()
        .map(|m| (m.id, m))
        .collect::<HashMap<&str, Module>>();

    let conjunctions: Vec<&str> = module_map
        .iter()
        .filter(|(_, m)| matches!(m.module_type, ModuleType::Conjunction { .. }))
        .map(|(id, _)| *id)
        .collect();

    let all_inputs: HashSet<&str> = module_map.keys().copied().collect();
    let all_outputs: HashSet<&str> = module_map
        .iter()
        .flat_map(|(_, m)| m.outputs.iter().copied())
        .collect();
    for module in all_outputs.difference(&all_inputs) {
        module_map.entry(module).or_insert(Module {
            id: module,
            module_type: ModuleType::Test,
            outputs: Vec::default(),
        });
    }

    for conjunction in conjunctions {
        let inputs: Vec<&str> = module_map
            .iter()
            .filter(|(_, m)| m.outputs.contains(&conjunction))
            .map(|(id, _)| *id)
            .collect();

        for input in inputs {
            module_map.entry(conjunction).and_modify(|m| {
                if let ModuleType::Conjunction { inputs, .. } = &mut m.module_type {
                    inputs.insert(input, Signal::Low);
                }
            });
        }
    }

    Ok((input, module_map))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, mut modules) = parser(input).unwrap();
    debug_assert!(input.is_empty());
    let mut low_signals = 0;
    let mut high_signals = 0;
    for _ in 0..1000
    // For each button press
    {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", Signal::Low));
        low_signals += 1; // From initial button press
        loop {
            if queue.is_empty() {
                break;
            }
            let (from, to, signal) = queue.pop_front().unwrap();
            tracing::info!("{} -{:?}-> {}", from, signal, to);
            let module = modules.get_mut(to).unwrap();
            let signals = module.process(from, &signal);
            signals.iter().for_each(|(_, _, s)| match s {
                Signal::High => high_signals += 1,
                Signal::Low => low_signals += 1,
            });
            queue.extend(signals);
        }
    }

    Ok(low_signals * high_signals)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case("test-input.txt", 32000000)]
    #[case("test-input2.txt", 11687500)]
    fn test_process(#[case] filename: &str, #[case] expected: u64) -> miette::Result<()> {
        let input =
            String::from_utf8_lossy(&std::fs::read(std::path::Path::new(filename)).unwrap())
                .parse::<String>()
                .unwrap();
        assert_eq!(expected, process(&input)?);
        Ok(())
    }
}
