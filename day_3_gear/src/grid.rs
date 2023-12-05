#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Number,
    Symbol,
    Separator,
    Uninitialized,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    value: char,
    kind: TokenKind,
}
impl From<char> for Token {
    fn from(value: char) -> Self {
        let kind = match value {
            value if value.is_numeric() => TokenKind::Number,
            '.' => TokenKind::Separator,
            _ => TokenKind::Symbol,
        };

        Token { value, kind }
    }
}
impl Default for Token {
    fn default() -> Self {
        Token { value: '\0', kind: TokenKind::Uninitialized }
    }
}

#[derive(Debug)]
pub struct Grid<const W: usize, const H: usize> {
    tokens: [[Token; W]; H],
}
impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn get_token(&self, row_index: isize, col_index: isize) -> Option<&Token> {
        if row_index <= 0 || col_index <= 0 {
            return None;
        }
        let row_index = row_index as usize;
        let col_index = col_index as usize;


        match self.tokens.get(row_index) {
            Some(row) => match row.get(col_index) {
                Some(token) => Some(token),
                None => None,
            },
            None => None,
        }
    }
}
impl<const W: usize, const H: usize> std::str::FromStr for Grid<W, H> {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = [[Token::default(); W]; H];
        
        for (row_i, line) in s.lines().enumerate() {
            for (col_i, character) in line.chars().enumerate() {
                
                match tokens.get_mut(row_i) {
                    Some(row) => match row.get_mut(col_i) {
                        Some(token) => *token = character.into(),
                        None => return Err(format!("Input line {} is too long. expected each line to have {} elements but {} were found", row_i + 1, W, line.len()).into()),
                    },
                    None => return Err(format!("Too many lines in input. expected there to be {} lines", H).into()),
                }
            }
        }
        
        let grid = Grid { tokens };
        
        Ok(grid)
    }
}