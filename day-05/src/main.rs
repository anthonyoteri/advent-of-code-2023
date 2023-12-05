use std::collections::BTreeMap;

const SOIL: &'static str = "seed-to-soil";
const FERT: &'static str = "soil-to-fertilizer";
const WATER: &'static str = "fertilizer-to-water";
const LIGHT: &'static str = "water-to-light";
const TEMP: &'static str = "light-to-temperature";
const HUMIDITY: &'static str = "temperature-to-humidity";
const LOC: &'static str = "humidity-to-location";

fn extract_seeds(input: &str) -> Vec<usize> {
    let line = input.lines().nth(0).unwrap();
    let (_, values) = line.split_once(':').unwrap();
    values
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn extract_map_lines(input: &str, key: &'static str) -> Vec<String> {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();

    let start_idx = lines.iter().position(|line| line.starts_with(key)).unwrap();
    lines.drain(..=start_idx);

    if let Some(end_idx) = lines.iter().position(|line| line.is_empty()) {
        lines.drain(end_idx..);
    }

    lines
}

fn lookup(input: &str, source: usize, map: &'static str) -> usize {
    let map_lines = extract_map_lines(input, map);

    for line in map_lines {
        let values: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let dest_range_start = values[0];
        let source_range_start = values[1];
        let range_length = values[2];

        let source_range_end = source_range_start + range_length;
        if source >= source_range_start && source < source_range_end {
            return dest_range_start + (source - source_range_start);
        }
    }

    source
}

fn lookup_location(input: &str, seed: usize) -> usize {
    dbg!(&seed);
    let soil = lookup(input, seed, SOIL);
    dbg!(&soil);

    let fert = lookup(input, soil, FERT);
    dbg!(&fert);
    let water = lookup(input, fert, WATER);
    dbg!(&water);
    let light = lookup(input, water, LIGHT);
    dbg!(&light);
    let temp = lookup(input, light, TEMP);
    dbg!(&temp);
    let humidity = lookup(input, temp, HUMIDITY);
    dbg!(&humidity);
    let loc = lookup(input, humidity, LOC);
    dbg!(&loc);

    loc
}

fn part_1(input: &str) -> u32 {
    let seeds = extract_seeds(input);
    let mut locs = Vec::new();
    for seed in seeds {
        locs.push(lookup_location(input, seed));
    }
    locs.iter().min().unwrap().clone() as u32
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_seeds() {
        let input = include_str!("../test-input.txt");
        let seeds = extract_seeds(input);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_extract_map() {
        let input = include_str!("../test-input.txt");
        let lines = extract_map_lines(input, "seed-to-soil");
        assert_eq!(lines, vec!["50 98 2", "52 50 48"]);
    }

    #[test]
    fn test_lookup_location() {
        let input = include_str!("../test-input.txt");

        assert_eq!(lookup_location(input, 79), 82);
        assert_eq!(lookup_location(input, 14), 43);
        assert_eq!(lookup_location(input, 55), 86);
        assert_eq!(lookup_location(input, 13), 35);
    }

    #[test]
    fn test_lookup() {
        let input = include_str!("../test-input.txt");
        assert_eq!(lookup(input, 79, SOIL), 81);
        assert_eq!(lookup(input, 14, SOIL), 14);
        assert_eq!(lookup(input, 55, SOIL), 57);
        assert_eq!(lookup(input, 13, SOIL), 13);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../test-input.txt");
        assert_eq!(part_1(input), 35);
    }
}
