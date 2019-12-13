use console::{style, StyledObject};

use std::iter::empty;
use std::time::Duration;
use std::thread::sleep;
use std::fmt::Display;
use std::default::Default;
use std::collections::HashMap;

use boolinator::Boolinator;
use itertools::Itertools;
use aoc2019::intcode_full::{State, YieldReason, InputWord, word_narrow};

const INPUT: &'static str = include_str!("inputs/13.txt");
const FRAME_RATE: Duration = Duration::from_millis(100);
const SKIP_FRAME: usize = 100;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: InputWord,
    y: InputWord
}

type Tiles = HashMap<Pos, Tile>;

fn iw2t(iw: InputWord) -> Option<Tile> {
    match iw {
        0 => Some(Tile::Empty),
        1 => Some(Tile::Wall),
        2 => Some(Tile::Block),
        3 => Some(Tile::Paddle),
        4 => Some(Tile::Ball),
        _ => None
    }
}

fn tiles(state: &mut State) -> Option<(YieldReason, Option<InputWord>, Tiles)> {
    let (outputs, yield_reason) = state.run_to_yield();

    let mut score: Option<InputWord> = None;
    let mut tiles: Tiles = Default::default();

    for (xb, yb, tb) in outputs.into_iter().tuples() {
        let (x, y, t) = (word_narrow(xb)?, word_narrow(yb)?, word_narrow(tb)?);

        if x == -1 && y == 0 {
            score.replace(t);

        } else {
            let tile = iw2t(t)?;
            tiles.insert(Pos { x, y }, tile);
        }
    }

    Some((yield_reason, score, tiles))
}

fn merge_tiles(accum: &mut Tiles, new: &Tiles) {
    for (&pos, &tile) in new.iter() {
        accum.insert(pos, tile);
    }
}

fn draw_screen(tiles: &Tiles) {
    let (min_x, max_x) = tiles.iter().filter_map(|(&Pos { x, .. }, &v)| (v != Tile::Empty).as_some(x)).minmax().into_option().unwrap();
    let (min_y, max_y) = tiles.iter().filter_map(|(&Pos { y, .. }, &v)| (v != Tile::Empty).as_some(y)).minmax().into_option().unwrap();

    fn p<D: Display>(so: StyledObject<D>) {
        print!("{}", so);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let tile = tiles.get(&Pos { x, y }).cloned().unwrap_or(Tile::Empty);
            match tile {
                Tile::Empty => p(style(' ')),
                Tile::Wall => p(style('#').blue()),
                Tile::Block => p(style('#').cyan()),
                Tile::Paddle => p(style('=').white()),
                Tile::Ball => p(style('*').red())
            }
        }

        println!("");
    }
}

fn ball_and_paddle(tiles: &Tiles) -> Option<(Pos, Pos)> {
    let ball = tiles.iter().filter_map(|(&p, &v)| (v == Tile::Ball).as_some(p)).nth(0)?;
    let paddle = tiles.iter().filter_map(|(&p, &v)| (v == Tile::Paddle).as_some(p)).nth(0)?;
    Some((ball, paddle))
}

fn part_one() {
    let (yield_reason, _, tiles) = tiles(&mut State::new_from_str(INPUT, empty())).expect("Examples are correct!");
    assert_eq!(yield_reason, YieldReason::Halt);
    let block_tiles = tiles.values().filter(|&&t| t == Tile::Block).count();
    println!("{}", block_tiles);
}

fn part_two() {
    let mut state = State::new_from_str(INPUT, empty());
    state.write_memory(0, 2);

    let mut accum_tiles = Default::default();
    let mut accum_score = None;

    for frame in 0usize.. {
        let (yield_reason, score, tiles) = tiles(&mut state).expect("Examples are correct!");
        merge_tiles(&mut accum_tiles, &tiles);
        score.map(|score| accum_score.replace(score));

        let (ball, paddle) = ball_and_paddle(&accum_tiles).expect("There is ball and paddle!");

        let redraw = match yield_reason {
            YieldReason::Halt => true,
            YieldReason::WaitInput => {
                state.supply_input((ball.x - paddle.x).signum());
                frame % SKIP_FRAME == 0
            },
            _ => panic!("Incorrect IntCode program!")
        };

        if redraw {
            console::Term::stdout().clear_screen();

            println!("Score: {:?}", accum_score.unwrap_or(0));
            draw_screen(&accum_tiles);
            sleep(FRAME_RATE);
        }
    }
}

fn main() {
    part_one();
    part_two();
}
