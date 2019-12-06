use std::collections::VecDeque;
use std::collections::hash_map::HashMap;

use smallvec::SmallVec;
use boolinator::Boolinator;

const ADJ: usize = 2;
const INPUT: &'static str = include_str!("inputs/6.txt");

#[derive(Debug)]
struct Orbit {
    center: String,
    orbiting: String
}

type Graph = HashMap<String, SmallVec<[String; ADJ]>>;

fn parse_orbits(s: &str) -> Option<Vec<Orbit>> {
    s.lines()
        .map(str::trim)
        .map(|line| {
            let parts: SmallVec<[&str; 2]> = line.split(")").collect();
            (parts.len() == 2).as_some_from(|| {
                Orbit {
                    center: parts[0].to_string(),
                    orbiting: parts[1].to_string()
                }
            })
        })
        .collect()
}

fn orbits_to_graph(o: Vec<Orbit>) -> Graph {
    o.into_iter().fold(HashMap::new(), |mut graph, Orbit { center, orbiting }| {
        graph.entry(center.clone()).or_default().push(orbiting.clone());
        graph.entry(orbiting).or_default().push(center);
        graph
    })
}

fn bfs(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_front((start, 0usize));
    distances.insert(start.to_string(), 0usize);

    while let Some((node, depth)) = queue.pop_back() {
        for adj_node in graph.get(node).into_iter().flatten() {
            distances.entry(adj_node.to_string()).or_insert_with(|| {
                queue.push_front((adj_node, depth+1));
                depth+1
            });
        }
    }

    distances
}

fn solve_part_one(s: &str) -> usize {
    let orbits = parse_orbits(s).expect("Examples are correct.");
    let graph = orbits_to_graph(orbits);
    let distances = bfs(&graph, "COM");
    let total_orbits: usize = distances.values().sum();
    total_orbits
}

fn solve_part_two(s: &str) -> usize {
    let orbits = parse_orbits(s).expect("Examples are correct.");
    let graph = orbits_to_graph(orbits);
    let distances = bfs(&graph, "YOU");
    let route = distances.get("SAN").cloned().expect("Path exists");
    assert!(route >= 2);
    route - 2
}

#[test]
fn example_part_one() {
    let s = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    assert_eq!(solve_part_one(&s), 42);
}

#[test]
fn example_part_two() {
    let s = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    assert_eq!(solve_part_two(&s), 4);
}

fn part_one() {
    println!("{}", solve_part_one(INPUT));
}

fn part_two() {
    println!("{}", solve_part_two(INPUT));
}

fn main() {
    part_one();
    part_two();
}
