use std::cmp::{min, Ordering};
use std::collections::HashMap;

use num_integer::gcd;
use boolinator::Boolinator;
use itertools::Itertools;

const INPUT: &'static str = include_str!("inputs/10.txt");

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Roid {
    x: isize,
    y: isize
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Dxdy {
    dx: isize,
    dy: isize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4
}

impl Dxdy {
    fn minimal(&self) -> (Dxdy, isize) {
        let gcd = gcd(self.dx, self.dy);
        (Dxdy { dx: self.dx / gcd, dy: self.dy / gcd }, gcd)
    }

    fn dot(&self, other: Dxdy) -> isize {
        self.dx * other.dx + self.dy * other.dy
    }

    fn cross(&self, other: Dxdy) -> isize {
        self.dx * other.dy - self.dy * other.dx
    }

    fn quadrant_of(&self, other: Dxdy) -> Quadrant {
        match (self.dot(other), self.cross(other)) {
            (d, c) if d > 0 && c >= 0 => Quadrant::Q1,
            (d, c) if d <= 0 && c > 0 => Quadrant::Q2,
            (d, c) if d < 0 && c <= 0 => Quadrant::Q3,
            (d, c) if d >= 0 && c < 0 => Quadrant::Q4,
            _ => unreachable!()
        }
    }
}

#[test]
fn test_quadrant_of() {
    let up    = Dxdy { dx:  0, dy: -1 };
    let right = Dxdy { dx:  1, dy:  0 };
    let down  = Dxdy { dx:  0, dy:  1 };
    let left  = Dxdy { dx: -1, dy:  0 };

    let q1 = Dxdy { dx:  1, dy: -1 };
    let q2 = Dxdy { dx:  1, dy:  1 };
    let q3 = Dxdy { dx: -1, dy:  1 };
    let q4 = Dxdy { dx: -1, dy: -1 };

    assert_eq!(up.quadrant_of(right), Quadrant::Q2);
    assert_eq!(up.quadrant_of(down),  Quadrant::Q3);
    assert_eq!(up.quadrant_of(left),  Quadrant::Q4);
    assert_eq!(up.quadrant_of(up),    Quadrant::Q1);

    assert_eq!(up.quadrant_of(q1), Quadrant::Q1);
    assert_eq!(up.quadrant_of(q2), Quadrant::Q2);
    assert_eq!(up.quadrant_of(q3), Quadrant::Q3);
    assert_eq!(up.quadrant_of(q4), Quadrant::Q4);
}

impl Roid {
    fn dxdy_to(&self, other: Roid) -> Dxdy {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        Dxdy { dx, dy }
    }
}

fn parse(s: &str) -> impl Iterator<Item=Roid> + '_ {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                (c == '#').as_some(Roid { x: x as isize, y: y as isize })
            })
        })
}

fn to_minimal_dirs(from: Roid, locs: impl Iterator<Item=Roid>) -> impl Iterator<Item=(Dxdy, isize, Roid)> {
    locs.filter(move |&loc| loc != from)
        .map(move |loc| {
            let (dir, multiple) = from.dxdy_to(loc).minimal();
            (dir, multiple, loc)
        })
}

fn visible_from(from: Roid, locs: &[Roid]) -> Vec<Roid> {
    let mut visible: HashMap<Dxdy, (isize, Roid)> = HashMap::new();

    for (dir, multiple, loc) in to_minimal_dirs(from, locs.iter().cloned()) {
        let value = (multiple, loc);

        visible.entry(dir)
            .and_modify(|closest| { *closest = min(*closest, value); })
            .or_insert(value);
    }

    visible.values().map(|&(_, loc)| loc).collect()
}

fn rel_dir_comparator(rel: Dxdy, d1: Dxdy, d2: Dxdy) -> Ordering {
    rel.quadrant_of(d1).cmp(&rel.quadrant_of(d2))
        .then_with(|| d1.quadrant_of(d2).cmp(&d2.quadrant_of(d1)))
}

fn solve_part_one(s: &str) -> (usize, Roid) {
    let roids: Vec<Roid> = parse(s).collect();
    roids.iter().map(|&base| (visible_from(base, &roids).len(), base)).max_by_key(|t| t.0).expect("Examples are correct!")
}

fn solve_part_two(s: &str, from: Roid) -> Vec<Roid> {
    let mut roids: Vec<(Dxdy, isize, Roid)> = to_minimal_dirs(from, parse(s)).collect();

    let up = Dxdy { dx: 0, dy: -1 };
    roids.sort_unstable_by(|&(d1, l1, _), &(d2, l2, _)| {
        rel_dir_comparator(up, d1, d2).then_with(|| l1.cmp(&l2))
    });

    type Rays = Vec<(Dxdy, Vec<Roid>)>;

    let mut rays: Rays = roids.into_iter().rev().group_by(|&(d, _, _)| d)
        .into_iter()
        .map(|(d, group)| (d, group.map(|(_, _, r)| r).collect()))
        .collect();

    rays.reverse();

    let roids_left = |rays: &Rays| rays.iter().any(|&(_, ref ray)| !ray.is_empty());

    let mut order = Vec::new();
    while roids_left(&rays) {
        for &mut (_, ref mut rays) in rays.iter_mut() {
            if let Some(r) = rays.pop() {
                order.push(r);
            }
        }
    }

    order
}

#[test]
fn example_part_one_1() {
    let p = solve_part_one(&[
        ".#..#",
        ".....",
        "#####",
        "....#",
        "...##"
    ].join("\n"));

    assert_eq!(p, (8, Roid { x: 3, y: 4 }));
}

#[test]
fn example_part_one_2() {
    let p = solve_part_one(&[
        "......#.#.",
        "#..#.#....",
        "..#######.",
        ".#.#.###..",
        ".#..#.....",
        "..#....#.#",
        "#..#....#.",
        ".##.#..###",
        "##...#..#.",
        ".#....####"
    ].join("\n"));

    assert_eq!(p, (33, Roid { x: 5, y: 8 }));
}

#[test]
fn example_part_one_3() {
    let p = solve_part_one(&[
        "#.#...#.#.",
        ".###....#.",
        ".#....#...",
        "##.#.#.#.#",
        "....#.#.#.",
        ".##..###.#",
        "..#...##..",
        "..##....##",
        "......#...",
        ".####.###."
    ].join("\n"));

    assert_eq!(p, (35, Roid { x: 1, y: 2 }));
}

#[test]
fn example_part_one_4() {
    let p = solve_part_one(&[
        ".#..#..###",
        "####.###.#",
        "....###.#.",
        "..###.##.#",
        "##.##.#.#.",
        "....###..#",
        "..#.#..#.#",
        "#..#.#.###",
        ".##...##.#",
        ".....#.#.."
    ].join("\n"));

    assert_eq!(p, (41, Roid { x: 6, y: 3 }));
}

#[cfg(test)]
fn large_example() -> String {
    [
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##"
    ].join("\n")
}

#[test]
fn example_part_one_5() {
    let p = solve_part_one(&large_example());
    assert_eq!(p, (210, Roid { x: 11, y: 13 }));
}

#[test]
fn example_part_two_5() {
    let (_, ims) = solve_part_one(&large_example());
    let order = solve_part_two(&large_example(), ims);

    assert_eq!(order[0], Roid { x: 11, y: 12 });
    assert_eq!(order[1], Roid { x: 12, y: 1 });
    assert_eq!(order[2], Roid { x: 12, y: 2 });

    assert_eq!(order[9], Roid { x: 12, y: 8 });
    assert_eq!(order[19], Roid { x: 16, y: 0 });
    assert_eq!(order[49], Roid { x: 16, y: 9 });

    assert_eq!(order[99], Roid { x: 10, y: 16 });
    assert_eq!(order[198], Roid { x: 9, y: 6 });
    assert_eq!(order[199], Roid { x: 8, y: 2 });
    assert_eq!(order[200], Roid { x: 10, y: 9 });
    assert_eq!(order[298], Roid { x: 11, y: 1 });

    assert_eq!(order.len(), 299);
}

fn part_one() -> Roid {
    let (roids_cnt, roid) = solve_part_one(INPUT);
    println!("{}", roids_cnt);
    roid
}

fn part_two(ims: Roid) {
    let order = solve_part_two(INPUT, ims);
    let ans = order[199];
    println!("{}", ans.x * 100 + ans.y);
}

fn main() {
    let ims = part_one();
    part_two(ims);
}
