use std::cmp::max;

const INPUT: &'static str = include_str!("inputs/1.txt");

fn mass() -> impl Iterator<Item=u32> + Clone {
    INPUT.lines().map(|s| s.parse::<u32>().expect("unsigned integer"))
}

fn fuel(mass: u32) -> u32 {
    max(mass / 3, 2) - 2
}

fn fuel_rec(mut mass: u32) -> u32 {
    let mut total: u32 = 0;

    while mass > 0 {
        mass = fuel(mass);
        total += mass;
    }

    total
}

fn part_one() {
    let fuel_sum: u32 = mass().map(fuel).sum();
    println!("{}", fuel_sum);
}

fn part_two() {
    let fuel_rec_sum: u32 = mass().map(fuel_rec).sum();
    println!("{}", fuel_rec_sum);
}

fn main() {
    part_one();
    part_two();
}
