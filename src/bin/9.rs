use aoc2019::intcode_full::State;
use itertools::join;

const INPUT: &'static str = include_str!("inputs/9.txt");

fn part_one() {
    let outputs = State::outputs(INPUT, [1isize].iter().cloned());
    println!("{}", join(outputs, ","));
}

fn part_two() {
    let outputs = State::outputs(INPUT, [2isize].iter().cloned());
    println!("{}", join(outputs, ","));
}

fn main() {
    part_one();
    part_two();
}
