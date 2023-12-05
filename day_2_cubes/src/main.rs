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
        
        if game.is_possible() {
            sum += game.id();
            print!("  POSSIBLE ");
        } else {
            print!("IMPOSSIBLE ");
        }
        
        println!("{}", line);
    }

    println!("{}", sum);
}