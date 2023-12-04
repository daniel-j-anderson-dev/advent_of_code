mod lexer;
mod game;

use crate::{
    game::Game,
    lexer::Lexer
};

fn main() {
    let input = std::fs::read_to_string("day2.txt")
        .expect("Failed to open day 2 input");

    for line in input.lines() {
        let line_chars = line.chars().collect::<Vec<_>>();
        let line_tokens = Lexer::new(&line_chars).collect::<Vec<_>>();

        let Ok(game) = line.parse::<Game>() else { continue }; // each game is missing the last CubeSet

        println!("tokens: {:?}\nparsed: {:?}\n", line_tokens, game);
    }
}