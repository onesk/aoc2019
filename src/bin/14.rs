use std::convert::TryInto;
use std::collections::HashMap;

use boolinator::Boolinator;
use smallvec::SmallVec;

type Chemical = String;

const INPUT: &'static str = include_str!("inputs/14.txt");
const TRILLION: usize = 1_000_000_000_000;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Quantity {
    chemical: Chemical,
    quantity: usize
}

impl Quantity {
    fn parse(s: &str) -> Option<Self> {
        let parts: SmallVec<[&str; 2]> = s.splitn(2, ' ').collect();
        (parts.len() == 2).as_option()?;
        Some(Quantity { chemical: parts[1].to_string(), quantity: parts[0].parse().ok()? })
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Reaction {
    output: Quantity,
    inputs: Vec<Quantity>
}

impl Reaction {
    fn parse(line: &str) -> Option<Self> {
        let sides: SmallVec<[&str; 2]> = line.splitn(2, "=>").map(str::trim).collect();
        (sides.len() == 2).as_option()?;

        let inputs: Option<Vec<Quantity>> = sides[0].split(",")
            .map(str::trim)
            .map(Quantity::parse)
            .collect();

        let inputs = inputs?;
        let output = Quantity::parse(sides[1])?;

        Some(Reaction { output, inputs })
    }
}

type Reactions = HashMap<String, Reaction>;
type Lack = HashMap<String, isize>;

fn parse(s: &str) -> Option<Reactions> {
    s.lines().map(str::trim).map(|s| {
        let reaction = Reaction::parse(s)?;
        Some((reaction.output.chemical.clone(), reaction))
    }).collect()
}

fn batches(req: usize, batch: usize) -> (usize, usize) {
    let upper = req + batch - 1;
    let multiple = upper / batch;
    (multiple, multiple * batch)
}

#[test]
fn test_batches() {
    assert_eq!(batches(11, 3), (4, 12));
    assert_eq!(batches(13, 1), (13, 13));
}

fn expand_lack(rs: &Reactions, src: Chemical, end: Chemical, end_quantity: usize) -> Option<usize> {
    let mut lack: Lack = Lack::new();
    lack.insert(end, end_quantity as isize);

    while let Some((output, lacking_quantity)) = lack.iter_mut().filter(|(ref c, &mut q)| c != &&src && q > 0).nth(0) {
        let output = output.clone();
        let reaction = rs.get(&output)?;

        let (batches, output_delta) = batches(*lacking_quantity as usize, reaction.output.quantity);
        *lacking_quantity -= output_delta as isize;

        for Quantity { chemical: input, quantity: required_quantity } in &reaction.inputs {
            *lack.entry(input.clone()).or_insert(0) += (batches * required_quantity) as isize;
        }
    }

    lack.get(&src).cloned()?.try_into().ok()
}

fn required_ore(reactions: &Reactions, required_fuel: usize) -> Option<usize> {
    expand_lack(&reactions, "ORE".into(), "FUEL".into(), required_fuel)
}

fn part_one(s: &str) -> usize {
    let reactions = parse(s).expect("Parsing succeeds!");
    required_ore(&reactions, 1).expect("Sequence exists!")
}

fn part_two(s: &str) -> usize {
    let reactions = parse(s).expect("Parsing succeeds!");

    let (mut low, mut high) = (0isize, TRILLION as isize);

    let mut ret = None;
    while low <= high {
        let mid = (low + high) / 2;

        let ore = required_ore(&reactions, mid as usize).expect("Sequence exists!");

        if ore < TRILLION {
            ret.replace(mid as usize);
            low = mid + 1;

        } else {
            high = mid - 1;

        }
    }

    ret.expect("Cannot be None.")
}

#[test]
fn example_1() {
    let input = [
        "10 ORE => 10 A",
        "1 ORE => 1 B",
        "7 A, 1 B => 1 C",
        "7 A, 1 C => 1 D",
        "7 A, 1 D => 1 E",
        "7 A, 1 E => 1 FUEL"
    ].join("\n");

    assert_eq!(part_one(&input), 31);
}

#[test]
fn example_2() {
    let input = [
        "9 ORE => 2 A",
        "8 ORE => 3 B",
        "7 ORE => 5 C",
        "3 A, 4 B => 1 AB",
        "5 B, 7 C => 1 BC",
        "4 C, 1 A => 1 CA",
        "2 AB, 3 BC, 4 CA => 1 FUEL",
    ].join("\n");

    assert_eq!(part_one(&input), 165);
}

#[test]
fn example_3() {
    let input = [
        "157 ORE => 5 NZVS",
        "165 ORE => 6 DCFZ",
        "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
        "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
        "179 ORE => 7 PSHF",
        "177 ORE => 5 HKGWZ",
        "7 DCFZ, 7 PSHF => 2 XJWVT",
        "165 ORE => 2 GPVTF",
        "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    ].join("\n");

    assert_eq!(part_one(&input), 13312);
    assert_eq!(part_two(&input), 82892753);
}

#[test]
fn example_4() {
    let input = [
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
        "17 NVRVD, 3 JNWZP => 8 VPVL",
        "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
        "22 VJHF, 37 MNCFX => 5 FWMGM",
        "139 ORE => 4 NVRVD",
        "144 ORE => 7 JNWZP",
        "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
        "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
        "145 ORE => 6 MNCFX",
        "1 NVRVD => 8 CXFTF",
        "1 VJHF, 6 MNCFX => 4 RFSQX",
        "176 ORE => 6 VJHF",
    ].join("\n");

    assert_eq!(part_one(&input), 180697);
    assert_eq!(part_two(&input), 5586022);
}

#[test]
fn example_5() {
    let input = [
        "171 ORE => 8 CNZTR",
        "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
        "114 ORE => 4 BHXH",
        "14 VRPVC => 6 BMBT",
        "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
        "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
        "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
        "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
        "5 BMBT => 4 WPTQ",
        "189 ORE => 9 KTJDG",
        "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
        "12 VRPVC, 27 CNZTR => 2 XDBXC",
        "15 KTJDG, 12 BHXH => 5 XCVML",
        "3 BHXH, 2 VRPVC => 7 MZWV",
        "121 ORE => 7 VRPVC",
        "7 XCVML => 6 RJRHP",
        "5 BHXH, 4 VRPVC => 5 LTCX",
    ].join("\n");

    assert_eq!(part_one(&input), 2210736);
    assert_eq!(part_two(&input), 460664);
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}
