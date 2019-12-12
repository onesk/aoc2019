use std::iter::empty;
use std::convert::{From, Into};
use std::collections::HashMap;

use boolinator::Boolinator;
use itertools::{Itertools, join};

use aoc2019::intcode_full::{State, YieldReason, InputWord, word_narrow};

const INPUT: &'static str = include_str!("inputs/11.txt");

#[derive(Copy, Clone)]
struct Dir {
    dx: isize,
    dy: isize
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    Black,
    White
}

impl From<InputWord> for Color {
    fn from(c: InputWord) -> Color {
        match c {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unknown color!")
        }
    }
}

impl Into<InputWord> for Color {
    fn into(self) -> InputWord {
        match self {
            Color::Black => 0,
            Color::White => 1
        }
    }
}

impl Dir {
    fn rotate_left(self) -> Self {
        Self { dx: self.dy, dy: -self.dx }
    }

    fn rotate_right(self) -> Self {
        Self { dx: -self.dy, dy: self.dx  }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pixel {
    x: isize,
    y: isize
}

impl Pixel {
    fn step(self, dir: Dir) -> Self {
        Self { x: self.x + dir.dx, y: self.y + dir.dy }
    }
}

struct Bitmap(HashMap<Pixel, Color>);

impl Bitmap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn paint(&mut self, p: Pixel, color: Color) {
        self.0.insert(p, color);
    }

    fn at(&self, p: Pixel) -> Color {
        self.0.get(&p).cloned().unwrap_or(Color::Black)
    }

    fn painted(&self) -> usize {
        self.0.len()
    }

    fn white_pixels(&self) -> impl Iterator<Item=Pixel> + '_ {
        self.0.iter().filter_map(|(&p, &c)| (c == Color::White).as_some(p))
    }
}

fn produce_bitmap(s: &str, start_color: Color) -> Bitmap {
    let mut state = State::new_from_str(s, empty());
    let mut bitmap = Bitmap::new();

    let mut pos = Pixel { x: 0, y: 0 };
    let mut dir = Dir { dx: 0, dy: -1 };

    bitmap.paint(pos, start_color);

    loop {
        state.supply_input(bitmap.at(pos).into());

        let (outputs, yield_reason) = state.run_to_yield();

        match yield_reason {
            YieldReason::Halt => break,
            YieldReason::WaitInput => {
                assert!(outputs.len() == 2);

                let color: Color = word_narrow(outputs[0].clone()).expect("Cannot narrow!").into();

                bitmap.paint(pos, color);

                match word_narrow(outputs[1].clone()) {
                    Some(0) => dir = dir.rotate_left(),
                    Some(1) => dir = dir.rotate_right(),
                    _ => panic!("Incorrect direction!")
                }

                pos = pos.step(dir);
            },
            _ => panic!("Incorrect program flow!")
        }

    }

    bitmap
}

fn part_one() {
    let bitmap = produce_bitmap(INPUT, Color::Black);
    println!("{}", bitmap.painted());
}

fn part_two() {
    let bitmap = produce_bitmap(INPUT, Color::White);

    let (min_x, max_x) = bitmap.white_pixels().map(|Pixel { x, .. }| x).minmax().into_option().unwrap();
    let (min_y, max_y) = bitmap.white_pixels().map(|Pixel { y, .. }| y).minmax().into_option().unwrap();

    for y in min_y..=max_y {
        let line = (min_x..=max_x)
            .map(|x| match bitmap.at(Pixel { x ,y }) {
                Color::White => '#',
                Color::Black => ' '
            });

        println!("{}", join(line, ""));
    }
}

fn main() {
    part_one();
    part_two();
}
