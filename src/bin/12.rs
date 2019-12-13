use std::collections::HashMap;
use smallvec::SmallVec;
use num_integer::lcm;

const NDIM: usize = 3;
const NBODY: usize = 4;

type Coord = isize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pos: Coord,
    vel: Coord,
}

type CoordState = SmallVec<[State; NBODY]>;
type CoordStates = SmallVec<[CoordState; NDIM]>;

fn simulate_step_1d(s: &mut CoordState) {
    let delta_vel: SmallVec<[Coord; NBODY]> = s.iter().map(|s1| {
        s.iter().map(|s2| (s2.pos - s1.pos).signum()).sum()
    }).collect();

    for (s, delta_vel) in s.iter_mut().zip(delta_vel) {
        s.vel += delta_vel;
        s.pos += s.vel;
    }
}

fn loop_length_1d(s: &mut CoordState) -> usize {
    let mut seen: HashMap<CoordState, usize> = HashMap::new();

    for step in 0.. {
        if let Some(start_step) = seen.remove(s) {
            return step - start_step;
        }

        seen.insert(s.clone(), step);
        simulate_step_1d(s);
    }

    unreachable!();
}

fn coord_states<'a>(npos: impl Iterator<Item=&'a [Coord; NDIM]> + 'a) -> CoordStates {
    let mut ret = CoordStates::new();

    for _ in 0..NDIM {
        ret.push(CoordState::new())
    }

    for pos in npos {
        for j in 0..NDIM {
            ret[j].push(State { pos: pos[j], vel: 0 });
        }
    }

    ret
}

fn total_energy_after<'a>(mut coords: CoordStates, steps: usize) -> Coord {
    for j in 0..NDIM {
        for _ in 0..steps {
            simulate_step_1d(&mut coords[j]);
        }
    }

    (0..NBODY).map(|i| {
        let potential: Coord = (0..NDIM).map(|j| coords[j][i].pos.abs()).sum();
        let kinetic: Coord = (0..NDIM).map(|j| coords[j][i].vel.abs()).sum();

        potential * kinetic
    }).sum()
}

fn loop_length(mut coords: CoordStates) -> usize {
    (0..NDIM).fold(1, |ret, j| lcm(ret, loop_length_1d(&mut coords[j])))
}

#[test]
fn example_part_one_1() {
    let moons = [
        [-1, 0, 2],
        [2, -10, -7],
        [4, -8, 8],
        [3, 5, -1]
    ];

    assert_eq!(total_energy_after(coord_states(moons[..].iter()), 10), 179);
}

#[cfg(test)]
fn example_2<'a>() -> impl Iterator<Item=&'a [Coord; NDIM]> + 'a {
    [
        [-8, -10, 0],
        [5, 5, 10],
        [2, -7, 3],
        [9, -8, -3]
    ].iter()
}

#[test]
fn example_part_one_2() {
    assert_eq!(total_energy_after(coord_states(example_2()), 100), 1940);
}

#[test]
fn example_part_two_2() {
    assert_eq!(loop_length(coord_states(example_2())), 4686774924);
}

fn input<'a>() -> impl Iterator<Item=&'a [Coord; NDIM]> + 'a {
    [
        [5, -1, 5],
        [0, -14, 2],
        [16, 4, 0],
        [18, 1, 16]
    ].iter()
}

fn part_one() {
    println!("{}", total_energy_after(coord_states(input()), 1000));
}

fn part_two() {
    println!("{}", loop_length(coord_states(input())));
}

fn main() {
    part_one();
    part_two();
}
