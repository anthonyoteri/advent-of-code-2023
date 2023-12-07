use day_07::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input.txt");
    part1::process(divan::black_box(input)).unwrap();
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input.txt");
    part2::process(divan::black_box(input)).unwrap();
}
