use std::collections::HashMap;

use smallvec::SmallVec;
use boolinator::Boolinator;
use lazy_static::lazy_static;

use num_traits::cast::ToPrimitive;
use num_integer::Integer;
use num_bigint::{BigUint, BigInt, ToBigInt, ToBigUint};

const MAX_PARAMS: usize = 3;
const INPUTS: usize = 2;
const OUTPUTS: usize = 4;

pub type Pos = BigUint;
pub type Word = BigInt;
pub type InputWord = isize;

pub struct Memory(HashMap<Pos, Word>);

pub fn word_narrow(w: Word) -> Option<InputWord> {
    w.to_isize()
}

impl Memory {
    fn get(&self, index: &Pos) -> Word {
        self.0.get(index).cloned().unwrap_or_else(|| 0isize.into())
    }

    fn get_mut(&mut self, index: Pos) -> &mut Word {
        self.0.entry(index).or_insert_with(|| 0isize.into())
    }
}

pub struct State {
    ip: Pos,
    rel_base: Word,
    halted: bool,
    memory: Memory,
    inputs: SmallVec<[Word; INPUTS]>,
    outputs: SmallVec<[Word; OUTPUTS]>
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
    Equals,
    AdjRelBase
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum YieldReason {
    IncorrectOpcode,
    NegativeAddress,
    NoSuchArg,
    WaitInput,
    Halt
}

impl Opcode {
    fn parse(opcode: usize) -> Option<(Self, usize)> {
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
            9 => Some((Opcode::AdjRelBase, 1)),
            _ => None
        }
    }
}

#[derive(Clone)]
enum Arg {
    Immediate(Word),
    Position(Pos),
    Relative(Word)
}

impl Arg {
    fn parse(im: Pos, arg: Word) -> Option<Self> {
        match im.to_usize()? {
            0 => Some(Arg::Position(arg.to_biguint()?)),
            1 => Some(Arg::Immediate(arg)),
            2 => Some(Arg::Relative(arg)),
            _ => None
        }
    }
}

struct Insn {
    opcode: Opcode,
    args: SmallVec<[Arg; MAX_PARAMS]>
}

lazy_static! {
    static ref BUI_100: BigUint = 100usize.to_biguint().unwrap();
    static ref BUI_10: BigUint = 10usize.to_biguint().unwrap();
}

impl Insn {
    fn parse(memory: &Memory, pos: &Pos) -> Option<Insn> {
        let insn = memory.get(pos).to_biguint()?;

        let (mut insn, opcode) = insn.div_rem(&*BUI_100);
        let (opcode, arg_count) = Opcode::parse(opcode.to_usize()?)?;

        let mut args: SmallVec<_> = SmallVec::new();
        for i in 1..=arg_count {
            let arg = memory.get(&(pos + i));
            let (insn_, im) = insn.div_rem(&*BUI_10);
            insn = insn_;

            args.push(Arg::parse(im, arg)?)
        }

        Some(Insn { opcode, args })
    }

    fn get_arg(&self, pos: usize) -> Result<Arg, YieldReason> {
        self.args.get(pos).cloned().ok_or(YieldReason::NoSuchArg)
    }
}

fn parse_program(s: &str) -> Option<Memory> {
    let str_slices = s.trim().split(",").enumerate();
    let bigints = str_slices.map(|(i, s)| {
        let smallint: isize = s.parse().ok()?;
        Some((i.to_biguint()?, smallint.to_bigint()?))
    });

    let hashmap: Option<_> = bigints.collect();
    Some(Memory(hashmap?))
}

impl State {
    pub fn new_from_str(s: &str, inputs: impl Iterator<Item=InputWord>) -> Self {
        let memory = parse_program(s).expect("Examples are correct.");

        let mut inputs: SmallVec<_> = inputs.map(|iw| iw.into()).collect();
        inputs.reverse();

        State {
            ip: 0usize.to_biguint().unwrap(),
            rel_base: 0isize.to_bigint().unwrap(),
            halted: false,
            outputs: SmallVec::new(),
            inputs,
            memory
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn outputs(s: &str, inputs: impl Iterator<Item=InputWord>) -> Vec<Word> {
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

    pub fn supply_input(&mut self, input: InputWord) {
        self.inputs.insert(0, input.into())
    }

    pub fn step(&mut self) -> Result<(), YieldReason> {
        (!self.halted).ok_or(YieldReason::Halt)?;

        let insn = Insn::parse(&self.memory, &self.ip).ok_or(YieldReason::IncorrectOpcode)?;

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
                    Opcode::JumpIfTrue => op.to_isize() != Some(0isize),
                    Opcode::JumpIfFalse => op.to_isize() == Some(0isize),
                    _ => unreachable!()
                };

                flag.as_some(npos.to_biguint().ok_or(YieldReason::IncorrectOpcode)?)
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

                *dest = (flag as isize).to_bigint().expect("Never fails.");

                None
            },

            Opcode::AdjRelBase => {
                let op = self.in_arg(insn.get_arg(0)?)?;
                self.rel_base += op;

                None
            }
        };

        self.ip = npos.unwrap_or_else(|| self.ip.clone() + 1usize + insn.args.len());
        Ok(())
    }

    fn in_arg(&self, arg: Arg) -> Result<Word, YieldReason> {
        match arg {
            Arg::Immediate(val) => Ok(val),
            Arg::Position(pos) => Ok(self.memory.get(&pos)),
            Arg::Relative(offset) => Ok(self.memory.get(&self.rel_addr(offset)?))
        }
    }

    fn out_arg(&mut self, arg: Arg) -> Result<&mut Word, YieldReason> {
        match arg {
            Arg::Immediate(_) => Err(YieldReason::IncorrectOpcode),
            Arg::Position(pos) => Ok(self.memory.get_mut(pos)),
            Arg::Relative(offset) => Ok(self.memory.get_mut(self.rel_addr(offset)?))
        }
    }

    fn rel_addr(&self, offset: Word) -> Result<Pos, YieldReason> {
        (self.rel_base.clone() + offset).to_biguint().ok_or(YieldReason::NegativeAddress)
    }

    fn run_to_halt(&mut self) {
        while !self.halted {
            self.step().expect("Examples are correct!");
        }
    }
}

#[test]
fn quine() {
    let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let outputs = State::outputs(&quine, [0isize].into_iter().cloned());
    assert_eq!(itertools::join(&outputs, ","), quine);
}

#[test]
fn bigint() {
    let code = "1102,34915192,34915192,7,4,7,99,0";
    let outputs = State::outputs(&code, [0isize].into_iter().cloned());
    assert_eq!(itertools::join(&outputs, ",").len(), 16);
}

#[test]
fn bigint_another() {
    let code = "104,1125899906842624,99";
    let outputs = State::outputs(&code, [0isize].into_iter().cloned());
    assert_eq!(itertools::join(&outputs, ","), "1125899906842624");
}
