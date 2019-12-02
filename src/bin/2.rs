#[allow(unused_imports)]

use std::convert::TryInto;
use itertools::{join, iproduct};
use boolinator::Boolinator;

const INPUT: &'static str = include_str!("inputs/2.txt");

type Memory = Vec<usize>;

fn parse_line(s: &str) -> Option<Memory> {
    s.trim().split(",").map(|l| l.parse().ok()).collect()
}

struct State {
    pos: usize,
    halted: bool,
    memory: Memory
}

impl State {
    fn new_from_str(s: &str) -> Self {
        let memory = parse_line(s).expect("Examples are correct.");
        State { pos: 0, halted: false, memory }
    }

    fn step(&mut self) {
        assert!(!self.halted);

        let opcode = self.memory[self.pos];
        let operands: Option<[usize; 3]> = self.memory[self.pos..].get(1..4).map(|s| s.try_into().unwrap());

        if opcode == 99 {
            self.halted = true;
            return;
        }

        if let Some([op1, op2, dest]) = operands {
            let op1 = self.memory[op1];
            let op2 = self.memory[op2];

            self.memory[dest] = match opcode {
                1 => op1 + op2,
                2 => op1 * op2,
                _ => panic!("Unknown opcode!")
            };

            self.pos += 4;

        } else {
            panic!("Incorrect length!");
        }
    }

    fn run_to_halt(&mut self) {
        while !self.halted {
            self.step()
        }
    }

    #[allow(dead_code)]
    fn str_state(&self) -> String {
        join(&self.memory, ",")
    }
}

#[cfg(test)]
fn final_state(s: &str) -> String {
    let mut state = State::new_from_str(s);
    state.run_to_halt();
    state.str_state()
}

#[test]
fn example_final_states() {
    assert_eq!(final_state("1,9,10,3,2,3,11,0,99,30,40,50"), "3500,9,10,70,2,3,11,0,99,30,40,50");
    assert_eq!(final_state("1,0,0,0,99"), "2,0,0,0,99");
    assert_eq!(final_state("2,3,0,3,99"), "2,3,0,6,99");
    assert_eq!(final_state("2,4,4,5,99,0"), "2,4,4,5,99,9801");
    assert_eq!(final_state("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
}

#[derive(Copy, Clone)]
struct Params {
    noun: usize,
    verb: usize
}

impl Params {
    fn enumerate() -> impl Iterator<Item=Params> {
        iproduct!(0..=99, 0..=99).map(|(noun, verb)| Params { noun, verb })
    }
}

fn get_output(s: &str, params: Params) -> usize {
    let mut state = State::new_from_str(s);
    state.memory[1] = params.noun;
    state.memory[2] = params.verb;

    state.run_to_halt();

    state.memory[0]
}

fn part_one() {
    println!("{}", get_output(INPUT, Params { noun: 12, verb: 2 }));
}

fn find_noun_verb() -> Option<Params> {
    Params::enumerate().filter_map(|p| (get_output(INPUT, p) == 19690720).as_some(p)).nth(0)
}

fn part_two() {
    let solution = find_noun_verb().expect("Solution exists.");
    println!("{}", 100 * solution.noun + solution.verb);
}

fn main() {
    part_one();
    part_two();
}
