use smallvec::SmallVec;

const NDIM: usize = 3;
const NBODY: usize = 4;

type Coord = isize;

#[derive(Debug)]
struct State {
    pos: Coord,
    vel: Coord,
}

fn simulate_step_1d(s: &mut [State]) {
    let delta_vel: SmallVec<[Coord; NBODY]> = s.iter().map(|s1| {
        s.iter().map(|s2| (s2.pos - s1.pos).signum()).sum()
    }).collect();

    for (s, delta_vel) in s.iter_mut().zip(delta_vel) {
        s.vel += delta_vel;
        s.pos += s.vel;
    }
}

fn total_energy_after<'a>(npos: impl Iterator<Item=&'a [Coord; NDIM]> + 'a, steps: usize) -> Coord {
    let mut coords: SmallVec<[SmallVec<[State; NBODY]>; NDIM]> = SmallVec::new();

    for _ in 0..NDIM {
        coords.push(SmallVec::new())
    }

    for pos in npos {
        for j in 0..NDIM {
            coords[j].push(State { pos: pos[j], vel: 0 });
        }
    }

    for j in 0..NDIM {
        for _ in 0..steps {
            simulate_step_1d(&mut coords[j][..]);
        }
    }

    (0..NBODY).map(|i| {
        let potential: Coord = (0..NDIM).map(|j| coords[j][i].pos.abs()).sum();
        let kinetic: Coord = (0..NDIM).map(|j| coords[j][i].vel.abs()).sum();

        potential * kinetic
    }).sum()
}

#[test]
fn example_part_one_1() {
    let moons = [
        [-1, 0, 2],
        [2, -10, -7],
        [4, -8, 8],
        [3, 5, -1]
    ];

    assert_eq!(total_energy_after(moons[..].iter(), 10), 179);
}

#[test]
fn example_part_one_2() {
    let moons = [
        [-8, -10, 0],
        [5, 5, 10],
        [2, -7, 3],
        [9, -8, -3]
    ];

    assert_eq!(total_energy_after(moons[..].iter(), 100), 1940);
}

fn part_one() {
    let moons = [
        [5, -1, 5],
        [0, -14, 2],
        [16, 4, 0],
        [18, 1, 16]
    ];

    println!("{}", total_energy_after(moons[..].iter(), 1000));
}

fn main() {
    part_one();
}
