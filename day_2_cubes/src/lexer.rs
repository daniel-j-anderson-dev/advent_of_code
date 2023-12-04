#[derive(Debug)]
pub enum Token {
    Game(usize),
    Red(usize),
    Green(usize),
    Blue(usize),
    Other(String),
}
impl Token {
    pub fn from_raw(raw_token: &[char]) -> Option<Self> {
        if raw_token.starts_with(&['G', 'a', 'm', 'e']) {
            let slice = raw_token[4..].iter().collect::<String>().trim().to_owned();
            if let Ok(value) = slice.parse() {
                return Some(Token::Game(value));
            }
        }
        if raw_token.ends_with(&['r', 'e', 'd']) {
            let slice = raw_token[..raw_token.len() - 3].iter().collect::<String>().trim().to_owned();
            if let Ok(value) = slice.parse() {
                return Some(Token::Red(value));
            }
        }
        if raw_token.ends_with(&['g','r','e','e','n']) {
            let slice = raw_token[..raw_token.len() - 5].iter().collect::<String>().trim().to_owned();
            if let Ok(value) = slice.parse() {
                return Some(Token::Green(value));
            }
        }
        if raw_token.ends_with(&['b','l','u','e']) {
            let slice = raw_token[..raw_token.len() - 4].iter().collect::<String>().trim().to_owned();
            if let Ok(value) = slice.parse() {
                return Some(Token::Blue(value));
            }

        }

        Some(Token::Other(raw_token.iter().collect()))
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Game(id) => format!("Game {}:", id),
            Self::Red(val) => format!("{} red", val),
            Self::Blue(val) => format!("{} blue", val),
            Self::Green(val) => format!("{} green", val),
            Self::Other(s) => s.to_string(),
        })
    }
}

/// an iterator over tokens in a slice of chars
pub struct Lexer<'a> {
    content: &'a [char]
}
impl <'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Lexer<'a> {
        Lexer { content }
    }

    /// This function will truncate content at the given index and will return what was truncated
    fn cut(&mut self, index: usize) -> &'a [char] {
        let removed = &self.content[..index];
        self.content = &self.content[index..];
        removed
    }

    /// This function returns the index of the first character that makes the predicate false
    /// this function assumes the start of the token is `content[0]`
    fn find_token_end(&self, token_definition: fn(char) -> bool) -> usize {
        let mut token_end = 0;
        while token_end < self.content.len() && token_definition(self.content[token_end]) {
            token_end += 1;
        }
        token_end
    }

    /// This function truncates the left most token (ignoring whitespace) and returns it
    /// 
    /// 
    /// # Parameters 
    ///  - token_definition: a non environment capturing function that represents which characters are valid for a token
    fn cut_token(&mut self, token_definition: fn(char) -> bool) -> &'a [char] {
        self.cut(self.find_token_end(token_definition))
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // did we run out of tokens?
        if self.content.is_empty() {
            None
        }

        // is the token a stretch of whitespace?
        else if self.content[0].is_whitespace() {
            self.cut_token(|character| character.is_whitespace());
            self.next_token()
        }

        // is the token a number.
        else if self.content[0].is_numeric() {
            let number = self.cut_token(|character| character.is_numeric())
                .iter()
                .collect::<String>()
                .trim()
                .parse::<usize>()
                .expect("only numeric chars");

            self.cut_token(|character| character.is_whitespace());

            
            let color = self.cut_token(|character| character.is_alphabetic())
                .iter()
                .collect::<String>();
            
            let token = match color.as_str() {
                "red" => Token::Red(number),
                "green" => Token::Green(number),
                "blue" => Token::Blue(number),
                _ => Token::Other(format!("{} {}", number, color)),
            };

            Some(token)
        }

        // is the token a string
        else if self.content[0].is_alphabetic() {
            let game = self.cut_token(|character| character.is_alphabetic()).iter().collect::<String>();

            self.cut_token(|character| character.is_whitespace());

            let id = self.cut_token(|character| character.is_numeric())
                .iter()
                .collect::<String>()
                .trim()
                .parse::<usize>()
                .expect("only numeric characters");
            
            if game.as_str() == "Game" {
                Some(Token::Game(id))
            }
            else {
                Some(Token::Other(format!("{} {}", game, id)))
            }
        }

        else {
            let raw_token = self.cut(1).iter().collect::<String>();
            Some(Token::Other(raw_token))
            // self.next_token()
        }
    }
}
impl Into<Vec<String>> for Lexer<'_> {
    fn into(self) -> Vec<String> {
        self.map(|token| {
            token.to_string()
        })
        .collect()
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[test]
fn test() {
    let input = std::fs::read_to_string("../day2.txt").unwrap();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        let tokens = Lexer::new(&chars);

        for token in tokens {
            println!("{:?}", token);
        }
    }
}
