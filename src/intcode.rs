use std::convert::TryInto;

use smallvec::SmallVec;
use boolinator::Boolinator;

const MAX_PARAMS: usize = 3;
const INPUTS: usize = 2;
const OUTPUTS: usize = 4;

pub type Word = isize;
type Memory = Vec<Word>;

pub struct State {
    pos: usize,
    halted: bool,
    memory: Memory,
    inputs: SmallVec<[Word; INPUTS]>,
    outputs: SmallVec<[Word; OUTPUTS]>
}

fn divrem(i: Word, m: Word) -> (Word, Word) {
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum YieldReason {
    IncorrectOpcode,
    NoSuchArg,
    WaitInput,
    Halt
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
    Immediate(Word),
    Position(usize)
}

impl Arg {
    fn parse(im: Word, arg: Word) -> Option<Self> {
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
    fn parse(memory: &[Word]) -> Option<Insn> {
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

    fn get_arg(&self, pos: usize) -> Result<Arg, YieldReason> {
        self.args.get(pos).cloned().ok_or(YieldReason::NoSuchArg)
    }
}

fn parse_program(s: &str) -> Option<Memory> {
    s.trim().split(",").map(|l| l.parse().ok()).collect()
}

impl State {
    pub fn new_from_str(s: &str, inputs: impl Iterator<Item=Word>) -> Self {
        let memory = parse_program(s).expect("Examples are correct.");

        let mut inputs: SmallVec<_> = inputs.collect();
        inputs.reverse();

        State {
            pos: 0,
            halted: false,
            outputs: SmallVec::new(),
            inputs,
            memory
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn outputs(s: &str, inputs: impl Iterator<Item=Word>) -> Vec<Word> {
        let mut state = Self::new_from_str(s, inputs);
        state.run_to_halt();
        state.outputs.to_vec()
    }

    pub fn run_to_yield(&mut self) -> (Vec<Word>, YieldReason) {
        loop {
            if let Err(reason) = self.step() {
                let outputs = self.outputs.to_vec();
                self.outputs.clear();

                return (outputs, reason)
            }
        }
    }

    pub fn supply_input(&mut self, input: Word) {
        self.inputs.insert(0, input)
    }

    pub fn step(&mut self) -> Result<(), YieldReason> {
        (!self.halted).ok_or(YieldReason::Halt)?;

        let insn = Insn::parse(&self.memory[self.pos..]).ok_or(YieldReason::IncorrectOpcode)?;

        let npos = match insn.opcode {
            Opcode::Halt => {
                self.halted = true;
                None
            },

            Opcode::Add | Opcode::Mul => {
                let op1 = self.in_arg(insn.get_arg(0)?)?;
                let op2 = self.in_arg(insn.get_arg(1)?)?;
                let dest = self.out_arg(insn.get_arg(2)?)?;

                *dest = match insn.opcode {
                    Opcode::Add => op1 + op2,
                    Opcode::Mul => op1 * op2,
                    _ => unreachable!()
                };

                None
            },

            Opcode::Input => {
                let input = self.inputs.pop().ok_or(YieldReason::WaitInput)?;
                let dest = self.out_arg(insn.get_arg(0)?)?;
                *dest = input;

                None
            },

            Opcode::Output => {
                let op = self.in_arg(insn.get_arg(0)?)?;
                self.outputs.push(op);

                None
            },

            Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                let op = self.in_arg(insn.get_arg(0)?)?;
                let npos = self.in_arg(insn.get_arg(1)?)?;

                let flag = match insn.opcode {
                    Opcode::JumpIfTrue => op != 0,
                    Opcode::JumpIfFalse => op == 0,
                    _ => unreachable!()
                };

                flag.as_some(npos.try_into().ok().ok_or(YieldReason::IncorrectOpcode)?)
            },

            Opcode::LessThan | Opcode::Equals => {
                let op1 = self.in_arg(insn.get_arg(0)?)?;
                let op2 = self.in_arg(insn.get_arg(1)?)?;
                let dest = self.out_arg(insn.get_arg(2)?)?;

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
        Ok(())
    }

    fn in_arg(&self, arg: Arg) -> Result<Word, YieldReason> {
        match arg {
            Arg::Immediate(val) => Ok(val),
            Arg::Position(pos) => self.memory.get(pos).cloned().ok_or(YieldReason::NoSuchArg)
        }
    }

    fn out_arg(&mut self, arg: Arg) -> Result<&mut Word, YieldReason> {
        match arg {
            Arg::Immediate(_) => Err(YieldReason::IncorrectOpcode),
            Arg::Position(pos) => self.memory.get_mut(pos).ok_or(YieldReason::NoSuchArg)
        }
    }

    fn run_to_halt(&mut self) {
        while !self.halted {
            self.step().expect("Examples are correct!");
        }
    }
}
