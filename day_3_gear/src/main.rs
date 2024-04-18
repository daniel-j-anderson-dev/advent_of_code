mod grid;
mod lexer;

use grid::{Grid, TokenKind};
use lexer::Lexer;

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("day3.txt")?;
    let lexer = Lexer::new(&input);

    for token in lexer {
        println!("{}", token);
    }

    Ok(())
}
