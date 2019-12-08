use itertools::join;

const INPUT: &'static str = include_str!("inputs/8.txt");

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn parse_input(s: &str) -> Option<Vec<u8>> {
    s.trim().chars().map(|c| c.to_digit(10).map(|d| d as u8)).collect()
}

fn part_one() {
    let image = parse_input(INPUT).expect("Examples are correct!");
    let layers = image.chunks(WIDTH * HEIGHT);

    let least_zeros = layers
        .min_by_key(|&layer| layer.iter().filter(|&&d| d == 0).count())
        .expect("At least one layer.");

    let num_1 = least_zeros.iter().filter(|&&d| d == 1).count();
    let num_2 = least_zeros.iter().filter(|&&d| d == 2).count();

    println!("{:?}", num_1 * num_2);
}

fn part_two() {
    let image = parse_input(INPUT).expect("Examples are correct!");
    let layers = image.chunks(WIDTH * HEIGHT);

    let mut decoded = [0u8; WIDTH * HEIGHT];

    for layer in layers.rev() {
        for (d, &l) in decoded.iter_mut().zip(layer.iter()) {
            if l != 2 {
                *d = l;
            }
        }
    }

    for row in decoded.chunks(WIDTH) {
        println!("{:?}", join(row.iter().map(|&d| if d == 1 { "#" } else { " " }), ""));
    }
}

fn main() {
    part_one();
    part_two();
}
