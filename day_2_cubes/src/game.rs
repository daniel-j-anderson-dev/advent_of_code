use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}
impl CubeSet {
    pub fn is_possible(&self, max: CubeSet) -> bool {
        self.red <= max.red && self.blue <= max.blue && self.green <= max.green
    }
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}
impl Game {
    const MAX: CubeSet = CubeSet {
        red: 12,
        blue: 14,
        green: 13,
    };
    // The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
    pub fn is_possible(&self) -> bool {
        for set in self.sets.iter() {
            if !set.is_possible(Game::MAX) {
                return false
            }
        }
        true
    }
    pub fn lowest_possible_max(&self) -> CubeSet {
        todo!();
    }
    pub fn id(&self) -> usize {
        self.id
    } 
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
                    } else {
                        return Err("A game id already exists. More than one Game token".into())
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
                Token::Other(separator) =>  {
                    if separator == ";" {
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
                    continue;
                },
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

        let game = Game {
            id: match game_id {
                Some(id) => *id,
                None => return Err("No game id. missing Game token".into()),
            },
            sets,
        };

        Ok(game)
    }
}

