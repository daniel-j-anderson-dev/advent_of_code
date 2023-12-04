use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}
impl CubeSet {
    pub fn new(red_token: Token, blue_token: Token, green_token: Token) -> CubeSet {
        let Token::Red(red) = red_token else { panic!("Invalid red token") };
        let Token::Green(green) = green_token else { panic!("Invalid green token") };
        let Token::Blue(blue) = blue_token else { panic!("Invalid blue token") };

        CubeSet { red, blue, green }
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

        for token in tokens.iter() {
            
            if let Token::Game(id) = token {
                if game_id.is_none() {
                    game_id = Some(id);
                }
            }
            
            if let Token::Red(value) = token {
                red = *value;
            }
            if let Token::Green(value) = token {
                green = *value;
            }
            if let Token::Blue(value) = token {
                blue = *value;
            }

            if let Token::Other(s) = token {
                if s == ";" {
                    sets.push(
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
        }
        let Some(&id) = game_id else { return Err("No game id. missing Game token".into()) };

        let game = Game {
            id,
            sets,
        };

        Ok(game)
    }
}

