use smallvec::SmallVec;

use aoc2019::permutations::Permutations;
use aoc2019::intcode::{State, Word, YieldReason};

const INPUT: &'static str = include_str!("inputs/7.txt");

const STAGES: usize = 5;

type Perm = [usize; STAGES];
type PhaseSeq = [Word; STAGES];

fn perms() -> impl Iterator<Item=Perm> {
    Permutations::<Perm>::new()
}

fn map_perm<F: Fn(usize) -> Word>(i: Perm, f: F) -> PhaseSeq {
    let mut ret = [0; STAGES];
    for (dest, src) in ret.iter_mut().zip(i.iter()) {
        *dest = f(*src);
    }

    ret
}

fn run_amplifier(s: &str, phase_setting: Word, input: Word) -> Word {
    let mut outputs = State::outputs(s, [phase_setting, input].into_iter().cloned());
    assert!(outputs.len() == 1);
    outputs.pop().unwrap()
}

fn feedforward(s: &str, phase_seq: PhaseSeq) -> Word {
    let mut amps: SmallVec<[State; STAGES]> = phase_seq.into_iter()
        .map(|&phase_setting| State::new_from_str(s, [phase_setting].into_iter().cloned()))
        .collect();

    for ref mut amp in &mut amps {
        let (outputs, reason) = amp.run_to_yield();
        assert!(reason == YieldReason::WaitInput);
        assert!(outputs.is_empty());
    }

    let mut signal: Word = 0;

    loop {
        let mut reasons = SmallVec::<[YieldReason; STAGES]>::new();

        for ref mut amp in &mut amps {
            amp.supply_input(signal);
            let (outputs, reason) = amp.run_to_yield();

            reasons.push(reason);
            assert!(outputs.len() == 1);
            signal = outputs[0];
        }

        if reasons.iter().all(|&r| r == YieldReason::Halt) {
            break;
        } else if reasons.iter().any(|&r| r != YieldReason::WaitInput) {
            panic!("Inconsistent yield reasons.");
        }
    }

    signal
}

fn solve_part_one(s: &str) -> (Word, PhaseSeq) {
    perms()
        .map(|perm| map_perm(perm, |x| x as Word))
        .map(|ps| {
            let output = ps.into_iter().fold(0, |input: Word, &phase_setting| run_amplifier(s, phase_setting, input));
            (output, ps)
        })
        .max()
        .expect("Solution exists!")
}

fn solve_part_two(s: &str) -> (Word, PhaseSeq) {
    perms()
        .map(|perm| map_perm(perm, |x| (x + 5) as Word))
        .map(|ps| (feedforward(s, ps), ps))
        .max()
        .expect("Solution exists!")
}

#[test]
fn examples_part_one() {
    let ex1 = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    assert_eq!(solve_part_one(&ex1), (43210, [4, 3, 2, 1, 0]));

    let ex2 = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    assert_eq!(solve_part_one(&ex2), (54321, [0, 1, 2, 3, 4]));

    let ex3 = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    assert_eq!(solve_part_one(&ex3), (65210, [1, 0, 4, 3, 2]));
}

#[test]
fn examples_part_two() {
    let ex1 = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    assert_eq!(solve_part_two(&ex1), (139629729, [9, 8, 7, 6, 5]));
}

fn part_one() {
    let (signal, _) = solve_part_one(INPUT);
    println!("{}", signal);
}

fn part_two() {
    let (signal, _) = solve_part_two(INPUT);
    println!("{}", signal);
}

fn main() {
    part_one();
    part_two();
}
