#[derive(Debug, Clone, Copy, Default)]
pub enum TokenKind {
    Number,
    Symbol,
    Separator,

    #[default]
    Uninitialized,
}
impl TokenKind {
    pub fn is_number(&self) -> bool {
        if let TokenKind::Number { .. } = self {
            true
        } else {
            false
        }
    }
    pub fn is_symbol(&self) -> bool {
        if let TokenKind::Symbol = self {
            true
        } else {
            false
        }
    }
    pub fn is_separator(&self) -> bool {
        if let TokenKind::Separator = self {
            true
        } else {
            false
        }
    }
    pub fn is_initialized(&self) -> bool {
        if let TokenKind::Uninitialized = self {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Token {
    value: char,
    kind: TokenKind,
}
impl Token {
    pub fn value(&self) -> &char {
        &self.value
    }
    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}
impl From<char> for Token {
    fn from(value: char) -> Self {
        Token {
            value,
            kind: match value {
                value if value.is_numeric() => panic!("Did you check to the left and right for other numeric chars?!"),
                '.' => TokenKind::Separator,
                _ => TokenKind::Symbol,
            },
        }
    }
}

#[derive(Debug)]
pub struct Grid<const W: usize, const H: usize> {
    tokens: [[Token; W]; H],
}
impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn rows(&self) -> &[[Token; W]; H] {
        &self.tokens
    }
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
    pub fn is_token_a_part_number(&self, row_index: usize, col_index: usize) -> bool {
        let row_index = row_index as isize;
        let col_index = col_index as isize;

        let top_left = match self.get_token(row_index - 1, col_index - 1) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };
        let top = match self.get_token(row_index - 1, col_index) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };
        let top_right = match self.get_token(row_index - 1, col_index + 1) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };
        
        let left = match self.get_token(row_index, col_index - 1) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };
        let right = match self.get_token(row_index, col_index + 1) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };
        
        let bottom_left = match self.get_token(row_index + 1, col_index - 1) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };
        let bottom = match self.get_token(row_index + 1, col_index) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };
        let bottom_right = match self.get_token(row_index + 1, col_index + 1) {
            Some(token) => token.kind(),
            None => TokenKind::Uninitialized,
        };

        top_left.is_symbol()    || top.is_symbol()    || top_right.is_symbol() ||
        left.is_symbol()        ||   /*token*/           right.is_symbol()     ||
        bottom_left.is_symbol() || bottom.is_symbol() || bottom_right.is_symbol()

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