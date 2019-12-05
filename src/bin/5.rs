use std::convert::TryInto;

use smallvec::SmallVec;
use boolinator::Boolinator;

const INPUT: &'static str = include_str!("inputs/5.txt");

const MAX_PARAMS: usize = 3;
const OUTPUTS: usize = 8;

type Memory = Vec<isize>;

fn parse_line(s: &str) -> Option<Memory> {
    s.trim().split(",").map(|l| l.parse().ok()).collect()
}

struct State {
    pos: usize,
    halted: bool,
    memory: Memory,
    input: Option<isize>,
    outputs: SmallVec<[isize; OUTPUTS]>
}

fn divrem(i: isize, m: isize) -> (isize, isize) {
    (i % m, i / m)
}

#[derive(Debug)]
enum Opcode {
    Add,
    Mul,
    Halt,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals
}

impl Opcode {
    fn parse(opcode: isize) -> Option<(Self, usize)> {
        match opcode {
            99 => Some((Opcode::Halt, 0)),
            1 => Some((Opcode::Add, 3)),
            2 => Some((Opcode::Mul, 3)),
            3 => Some((Opcode::Input, 1)),
            4 => Some((Opcode::Output, 1)),
            5 => Some((Opcode::JumpIfTrue, 2)),
            6 => Some((Opcode::JumpIfFalse, 2)),
            7 => Some((Opcode::LessThan, 3)),
            8 => Some((Opcode::Equals, 3)),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
enum Arg {
    Immediate(isize),
    Position(usize)
}

impl Arg {
    fn parse(im: isize, arg: isize) -> Option<Self> {
        match im {
            0 => Some(Arg::Position(arg.try_into().ok()?)),
            1 => Some(Arg::Immediate(arg)),
            _ => None
        }
    }
}

struct Insn {
    opcode: Opcode,
    args: SmallVec<[Arg; MAX_PARAMS]>
}

impl Insn {
    fn parse(memory: &[isize]) -> Option<Insn> {
        let (insn, args) = memory.split_first()?;

        let (opcode, insn) = divrem(*insn, 100);
        let (opcode, arg_count) = Opcode::parse(opcode)?;

        let args: Option<_> = args.get(0..arg_count)?
            .iter()
            .scan(insn, |insn, &arg| {
                let (im, insn_) = divrem(*insn, 10);
                *insn = insn_;

                Some(Arg::parse(im, arg))
            })
            .collect();

        Some(Insn { opcode, args: args? })
    }
}

impl State {
    fn new_from_str(s: &str, input: isize) -> Self {
        let memory = parse_line(s).expect("Examples are correct.");
        State {
            pos: 0,
            halted: false,
            input: Some(input),
            outputs: SmallVec::new(),
            memory
        }
    }

    fn step(&mut self) -> Option<()>{
        (!self.halted).as_option()?;

        let insn = Insn::parse(&self.memory[self.pos..])?;

        let npos = match insn.opcode {
            Opcode::Halt => {
                self.halted = true;
                None
            },

            Opcode::Add | Opcode::Mul => {
                let op1 = self.in_arg(insn.args.get(0).cloned()?)?;
                let op2 = self.in_arg(insn.args.get(1).cloned()?)?;
                let dest = self.out_arg(insn.args.get(2).cloned()?)?;

                *dest = match insn.opcode {
                    Opcode::Add => op1 + op2,
                    Opcode::Mul => op1 * op2,
                    _ => unreachable!()
                };

                None
            },

            Opcode::Input => {
                let input = self.input.take()?;
                let dest = self.out_arg(insn.args.get(0).cloned()?)?;
                *dest = input;

                None
            },

            Opcode::Output => {
                let op = self.in_arg(insn.args.get(0).cloned()?)?;
                self.outputs.push(op);

                None
            },

            Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                let op = self.in_arg(insn.args.get(0).cloned()?)?;
                let npos = self.in_arg(insn.args.get(1).cloned()?)?;

                let flag = match insn.opcode {
                    Opcode::JumpIfTrue => op != 0,
                    Opcode::JumpIfFalse => op == 0,
                    _ => unreachable!()
                };

                flag.as_some(npos.try_into().ok()?)
            },

            Opcode::LessThan | Opcode::Equals => {
                let op1 = self.in_arg(insn.args.get(0).cloned()?)?;
                let op2 = self.in_arg(insn.args.get(1).cloned()?)?;
                let dest = self.out_arg(insn.args.get(2).cloned()?)?;

                let flag = match insn.opcode {
                    Opcode::LessThan => op1 < op2,
                    Opcode::Equals => op1 == op2,
                    _ => unreachable!()
                };

                *dest = flag as isize;

                None
            }
        };

        self.pos = npos.unwrap_or_else(|| self.pos + 1 + insn.args.len());
        Some(())
    }

    fn in_arg(&self, arg: Arg) -> Option<isize> {
        match arg {
            Arg::Immediate(val) => Some(val),
            Arg::Position(pos) => self.memory.get(pos).cloned()
        }
    }

    fn out_arg(&mut self, arg: Arg) -> Option<&mut isize> {
        match arg {
            Arg::Immediate(_) => None,
            Arg::Position(pos) => self.memory.get_mut(pos)
        }
    }

    fn run_to_halt(&mut self) {
        while !self.halted {
            self.step().expect("Examples are correct!");
        }
    }
}

fn outputs(s: &str, input: isize) -> Vec<isize> {
    let mut state = State::new_from_str(s, input);
    state.run_to_halt();
    state.outputs.to_vec()
}

#[test]
fn examples() {
    let ex1 = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    assert_eq!(outputs(ex1, 0), [0]);
    assert_eq!(outputs(ex1, 137), [1]);

    let ex2 = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    assert_eq!(outputs(ex2, 0), [0]);
    assert_eq!(outputs(ex2, 137), [1]);

    let larger_ex = ["3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
                     "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
                     "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"].concat();

    assert_eq!(outputs(&larger_ex, 5), [999]);
    assert_eq!(outputs(&larger_ex, 8), [1000]);
    assert_eq!(outputs(&larger_ex, 137), [1001]);
}

fn part_one() {
    println!("{:?}", outputs(INPUT, 1).last().expect("Exists."));
}

fn part_two() {
    println!("{:?}", outputs(INPUT, 5).last().expect("Exists."));
}

fn main() {
    part_one();
    part_two();
}
