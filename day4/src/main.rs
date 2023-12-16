mod card;
use card::Card;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let total_points = INPUT
        .lines()
        .map(|line| Card::from(line).points())
        .sum::<u32>();

    println!("{}", total_points);
}

