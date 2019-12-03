use std::cmp::{min, max};
use boolinator::Boolinator;
use itertools::iproduct;

const INPUT: &'static str = include_str!("inputs/3.txt");

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Segment(Dir, usize);

type Path = Vec<Segment>;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn manhattan_dist(&self, rhs: Point) -> usize {
        (self.x - rhs.x).abs() as usize + (self.y - rhs.y).abs() as usize
    }

    fn add_point(&self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }

    fn add_segment(&self, Segment(dir, dist): Segment) -> Self {
        let dist = dist as isize;

        let delta = match dir {
            Dir::Up => Point { x: 0, y: dist },
            Dir::Down => Point { x: 0, y: -dist },
            Dir::Right => Point { x: dist, y: 0 },
            Dir::Left => Point { x: -dist, y: 0 }
        };

        self.add_point(delta)
    }

}

struct Axis {
    min: isize,
    max: isize
}

struct AABB {
    x: Axis,
    y: Axis
}

impl Axis {
    fn from_coords(c1: isize, c2: isize) -> Axis {
        Axis { min: min(c1, c2), max: max(c1, c2) }
    }

    fn intersect(&self, rhs: &Axis) -> Option<Axis> {
        let n_min = max(self.min, rhs.min);
        let n_max = min(self.max, rhs.max);
        (n_min <= n_max).as_some_from(|| Axis { min: n_min, max: n_max })
    }

    fn point(&self) -> Option<isize> {
        (self.min == self.max).as_some(self.min)
    }
}

impl AABB {
    fn from_points(p1: Point, p2: Point) -> Self {
        let x = Axis::from_coords(p1.x, p2.x);
        let y = Axis::from_coords(p1.y, p2.y);
        AABB { x, y }
    }

    fn intersection_aabb(&self, rhs: &AABB) -> Option<AABB> {
        Some(AABB { x: self.x.intersect(&rhs.x)?, y: self.y.intersect(&rhs.y)? })
    }

    fn intersection_point(&self, rhs: &AABB) -> Option<Point> {
        let aabb = self.intersection_aabb(rhs)?;
        Some(Point { x: aabb.x.point()?, y: aabb.y.point()? })
    }
}

fn parse_dir(s: &str) -> Option<Segment> {
    let (dir, dist) = (s.get(..1)?, s.get(1..)?);

    let dist = dist.parse().ok()?;

    let dir = match dir {
        "U" => Some(Dir::Up),
        "D" => Some(Dir::Down),
        "L" => Some(Dir::Left),
        "R" => Some(Dir::Right),
        _ => None
    }?;

    Some(Segment(dir, dist))
}

fn parse_input(s: &str) -> Option<(Path, Path)> {
    let mut rows = s.trim().lines()
        .map(|line| line.trim().split(",").map(parse_dir).collect::<Option<Vec<_>>>())
        .collect::<Option<Vec<_>>>()?;

    let second = rows.pop()?;
    let first = rows.pop()?;

    rows.is_empty().as_some((first, second))
}

fn path_to_adj_points(p: &Path) -> Vec<(Point, Point)> {
    p.iter().scan(Point::origin(), |state, &segment| {
        let first_point = *state;
        *state = state.add_segment(segment);
        Some((first_point, *state))
    }).collect()
}

fn adj_points_to_aabb(ps: &[(Point, Point)]) -> Vec<AABB> {
    ps.iter().map(|&(p1, p2)| AABB::from_points(p1, p2)).collect()
}

fn intersections(w1: &[AABB], w2: &[AABB]) -> Vec<Point> {
    iproduct!(w1.iter(), w2.iter()).filter_map(|(a, b)| a.intersection_point(b)).collect()
}

fn min_manhattan(ps: &[Point]) -> Option<usize> {
    ps.iter().filter(|p| !p.is_origin()).map(|p| p.manhattan_dist(Point::origin())).min()
}

fn steps(ps: &[(Point, Point)], p: Point) -> Option<usize> {
    let p_aabb = AABB::from_points(p, p);

    let mut total = 0;
    for &(p1, p2) in ps.iter() {
        let intersects = AABB::from_points(p1, p2).intersection_point(&p_aabb).is_some();

        if intersects {
            return Some(total + p.manhattan_dist(p1));

        } else {
            total += p2.manhattan_dist(p1);

        }
    }

    None
}

fn min_steps(ps: &[Point], w1: &[(Point, Point)], w2: &[(Point, Point)]) -> Option<usize> {
    ps.iter().filter(|p| !p.is_origin()).filter_map(|&p| {
        let s1 = steps(w1, p)?;
        let s2 = steps(w2, p)?;
        Some(s1 + s2)
    }).min()
}

fn exec_part_one(s: &str) -> Option<usize> {
    let (wire1, wire2) = parse_input(s)?;

    let wire1_ap = path_to_adj_points(&wire1);
    let wire2_ap = path_to_adj_points(&wire2);

    let wire1_aabb = adj_points_to_aabb(&wire1_ap);
    let wire2_aabb = adj_points_to_aabb(&wire2_ap);

    let ipoints = intersections(&wire1_aabb, &wire2_aabb);

    min_manhattan(&ipoints)
}

fn exec_part_two(s: &str) -> Option<usize> {
    let (wire1, wire2) = parse_input(s)?;

    let wire1_ap = path_to_adj_points(&wire1);
    let wire2_ap = path_to_adj_points(&wire2);

    let wire1_aabb = adj_points_to_aabb(&wire1_ap);
    let wire2_aabb = adj_points_to_aabb(&wire2_ap);

    let ipoints = intersections(&wire1_aabb, &wire2_aabb);

    min_steps(&ipoints, &wire1_ap, &wire2_ap)
}

#[test]
fn examples() {
    let ex1 = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let ex2 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let ex3 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    assert_eq!(exec_part_one(ex1), Some(6));
    assert_eq!(exec_part_one(ex2), Some(159));
    assert_eq!(exec_part_one(ex3), Some(135));

    assert_eq!(exec_part_two(ex1), Some(30));
    assert_eq!(exec_part_two(ex2), Some(610));
    assert_eq!(exec_part_two(ex3), Some(410));
}

fn part_one() {
    println!("{:?}", exec_part_one(INPUT).expect("Challenge has solution."));
}

fn part_two() {
    println!("{:?}", exec_part_two(INPUT).expect("Challenge has solution."));
}

fn main() {
    part_one();
    part_two();
}
