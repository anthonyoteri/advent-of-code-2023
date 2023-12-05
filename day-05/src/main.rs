use indicatif::ProgressIterator;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

const SOIL: &str = "seed-to-soil";
const FERT: &str = "soil-to-fertilizer";
const WATER: &str = "fertilizer-to-water";
const LIGHT: &str = "water-to-light";
const TEMP: &str = "light-to-temperature";
const HUMIDITY: &str = "temperature-to-humidity";
const LOC: &str = "humidity-to-location";

fn extract_seeds(input: &str) -> Vec<usize> {
    let line = input.lines().next().unwrap();
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

fn lookup(maps: &BTreeMap<&'static str, Vec<String>>, source: usize, map: &'static str) -> usize {
    let map_lines = maps.get(map).unwrap();

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

fn rev_lookup(maps: &BTreeMap<&'static str, Vec<String>>, dest: usize, map: &'static str) -> usize {
    let map_lines = maps.get(map).unwrap();

    for line in map_lines {
        let values: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let dest_range_start = values[0];
        let source_range_start = values[1];
        let range_length = values[2];

        let dest_range_end = dest_range_start + range_length;
        if dest >= dest_range_start && dest < dest_range_end {
            return source_range_start + (dest - dest_range_start);
        }
    }

    dest
}

fn lookup_location(maps: &BTreeMap<&'static str, Vec<String>>, seed: usize) -> usize {
    let soil = lookup(maps, seed, SOIL);

    let fert = lookup(maps, soil, FERT);
    let water = lookup(maps, fert, WATER);
    let light = lookup(maps, water, LIGHT);
    let temp = lookup(maps, light, TEMP);
    let humidity = lookup(maps, temp, HUMIDITY);
    

    lookup(maps, humidity, LOC)
}

fn rev_lookup_location(maps: &BTreeMap<&'static str, Vec<String>>, loc: usize) -> usize {
    let humidity = rev_lookup(maps, loc, LOC);
    let temp = rev_lookup(maps, humidity, HUMIDITY);
    let light = rev_lookup(maps, temp, TEMP);
    let water = rev_lookup(maps, light, LIGHT);
    let fert = rev_lookup(maps, water, WATER);
    let soil = rev_lookup(maps, fert, FERT);
    

    rev_lookup(maps, soil, SOIL)
}

fn extract_maps(input: &str) -> BTreeMap<&'static str, Vec<String>> {
    let mut maps = BTreeMap::new();
    maps.insert(SOIL, extract_map_lines(input, SOIL));
    maps.insert(FERT, extract_map_lines(input, FERT));
    maps.insert(WATER, extract_map_lines(input, WATER));
    maps.insert(LIGHT, extract_map_lines(input, LIGHT));
    maps.insert(TEMP, extract_map_lines(input, TEMP));
    maps.insert(HUMIDITY, extract_map_lines(input, HUMIDITY));
    maps.insert(LOC, extract_map_lines(input, LOC));
    maps
}

fn part_1(input: &str) -> u32 {
    let maps = extract_maps(input);
    let seeds = extract_seeds(input);
    let mut locs = Vec::new();
    for seed in seeds {
        locs.push(lookup_location(&maps, seed));
    }
    *locs.iter().min().unwrap() as u32
}

fn part_2(input: &str) -> usize {
    let maps = extract_maps(input);
    let seed_pairs = extract_seeds(input);

    let mut ranges = Vec::new();
    for chunk in seed_pairs.chunks(2) {
        let start = chunk[0];
        let size = chunk[1];
        ranges.push(start..start + size);
    }

    let mut h2l = maps.get(LOC).unwrap().iter().collect::<BinaryHeap<_>>();

    let line = h2l.pop().unwrap();
    let values: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let dest_range_start = values[0];
    let range_length = values[2];

    let rng = 0..dest_range_start + range_length;

    rng.into_iter()
        .progress()
        .find(|loc| {
            let seed = rev_lookup_location(&maps, *loc);
            for range in &ranges {
                if range.contains(&seed) {
                    return true;
                }
            }
            false
        })
        .unwrap()
    //    0
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
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
        let maps = extract_maps(input);

        assert_eq!(lookup_location(&maps, 79), 82);
        assert_eq!(lookup_location(&maps, 14), 43);
        assert_eq!(lookup_location(&maps, 55), 86);
        assert_eq!(lookup_location(&maps, 13), 35);
    }

    #[test]
    fn test_reverse_lookup() {
        let input = include_str!("../test-input.txt");
        let maps = extract_maps(input);

        assert_eq!(rev_lookup_location(&maps, 82), 79);
        assert_eq!(rev_lookup_location(&maps, 43), 14);
        assert_eq!(rev_lookup_location(&maps, 86), 55);
        assert_eq!(rev_lookup_location(&maps, 35), 13);
    }

    #[test]
    fn test_lookup() {
        let input = include_str!("../test-input.txt");
        let maps = extract_maps(input);
        assert_eq!(lookup(&maps, 79, SOIL), 81);
        assert_eq!(lookup(&maps, 14, SOIL), 14);
        assert_eq!(lookup(&maps, 55, SOIL), 57);
        assert_eq!(lookup(&maps, 13, SOIL), 13);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../test-input.txt");
        assert_eq!(part_1(input), 35);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../test-input.txt");
        assert_eq!(part_2(input), 46);
    }
}
