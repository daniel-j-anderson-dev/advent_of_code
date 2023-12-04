use crate::lexer::Lexer;

#[derive(Debug)]
pub struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}
impl std::str::FromStr for CubeSet {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let tokens: Vec<String> = Lexer::new(&chars).into();
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
        let tokens: Vec<String> = Lexer::new(&chars).into();
        todo!();
    }
}

