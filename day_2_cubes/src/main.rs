mod lexer;
mod game;
use std::str::FromStr;

use crate::{
    game::Game,
    lexer::Lexer,
};

fn main() {
    let input = std::fs::read_to_string("day2.txt")
        .expect("Failed to open day 2 input");

    for line in input.lines() {
        let Ok(game) = Game::from_str(line) else { continue; };
        println!("{:?}", game);
    }
}