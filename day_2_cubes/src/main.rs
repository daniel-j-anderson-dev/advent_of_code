mod lexer;
mod game;

use crate::game::Game;

fn main() {
    let input = std::fs::read_to_string("day2.txt")
        .expect("Failed to open day 2 input");

    let mut sum = 0;

    for line in input.lines() {
        
        let game = match line.parse::<Game>() {
            Ok(game) => game,
            Err(game_parse_error) => {
                eprintln!("Invalid game: {}", game_parse_error);
                continue;
            },
        };

        let fewest_cubes = game.fewest_cubes_needed_to_make_possible();
        let power = fewest_cubes.power();

        sum += power;
        
        println!(
"{:?}
{}
fewest: {:?}
fewest power: {}
",
            game,
            if game.is_possible() { "POSSIBLE" } else { "IMPOSSIBLE" },
            fewest_cubes,
            power,
        );
    }

    println!("{}", sum);
}