use day_09::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box("../input.txt")).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box("../input.txt")).unwrap();
}
