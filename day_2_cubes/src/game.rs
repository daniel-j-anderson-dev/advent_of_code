use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}
impl CubeSet {
    pub fn is_possible(&self, max: CubeSet) -> bool {
        todo!();
    }
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

impl std::str::FromStr for Game {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let tokens: Vec<Token> = Lexer::new(&chars).collect();

        let mut sets = Vec::new();
        let mut game_id = None;

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for (token_index, token) in tokens.iter().enumerate() {
            match token {
                Token::Game(id) => {
                    if game_id.is_none() {
                       game_id = Some(id);
                    }
                },
                Token::Red(cube_count) => {
                    red = *cube_count;
                },
                Token::Green(cube_count) => {
                    green = *cube_count;
                },
                Token::Blue(cube_count) => {
                    blue = *cube_count;
                },
                Token::Other(separator)
                if separator == ";" =>  {
                    sets.push (
                        CubeSet {
                            red,
                            blue,
                            green,
                        }
                    );
                    red = 0;
                    green = 0;
                    blue = 0;
                },
                Token::Other(_) => continue,
            }

            if token_index == tokens.len() - 1 {
                sets.push (
                    CubeSet {
                        red,
                        blue,
                        green,
                    }
                );
                red = 0;
                green = 0;
                blue = 0;
            }
        }

        let Some(&id) = game_id else { return Err("No game id. missing Game token".into()) };

        let game = Game {
            id,
            sets,
        };

        Ok(game)
    }
}

