use smallvec::SmallVec;
use itertools::Itertools;
use boolinator::Boolinator;

const RADIX: usize = 10;
const DIGITS: usize = 6;

fn digits(mut password: usize) -> SmallVec<[u8; DIGITS]> {
    let mut ret = SmallVec::new();

    while password != 0 {
        ret.push((password % RADIX) as u8);
        password /= RADIX;
    }

    ret.reverse();
    ret
}

fn criteria_part_one(password: &usize) -> bool {
    let digits = digits(*password);
    let sorted = digits.iter().tuple_windows().all(|(a, b)| a <= b);
    let exists_equal = digits.iter().tuple_windows().any(|(a, b)| a == b);
    sorted && exists_equal
}

fn criteria_part_two(password: &usize) -> bool {
    let digits = digits(*password);
    let sorted = digits.iter().tuple_windows().all(|(a, b)| a <= b);

    let eligible_runs = digits
        .iter()
        .group_by(|&digit| digit)
        .into_iter()
        .filter_map(|(digit, group)| (group.count() == 2).as_some(digit))
        .count();

    sorted && eligible_runs > 0
}

fn part_one(passwords: impl Iterator<Item=usize>) {
    println!("{}", passwords.filter(criteria_part_one).count());
}

fn part_two(passwords: impl Iterator<Item=usize>) {
    println!("{}", passwords.filter(criteria_part_two).count());
}

fn main() {
    let range = 240920..=789857usize;
    part_one(range.clone().into_iter());
    part_two(range.into_iter());
}
