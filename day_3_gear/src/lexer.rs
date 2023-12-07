mod string_utilities;
use string_utilities::StringUtilities; // for taking slices of &str at char boundaries

// I made the StringUtilities trait
    // anything that implements the StringUtilities trait 
    // will get the `slice` method
// I implemented StringUtilities for str.
// so now we can call .slice(..) on any &str!!!!!!!

pub struct Lexer<'a> {
    content: &'a str,
}
impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer {
        Lexer { content }
    }

    fn cut_at(&mut self, index: usize) -> &'a str {
        let removed = self.content.slice(..index);
        self.content = self.content.slice(index..);
        removed
    }

    fn cut_uniform_token(&mut self, token_definition: fn(char) -> bool) -> &'a str {
        let token_end = self.find_uniform_token_end(token_definition);
        self.cut_at(token_end)
    }

    fn find_uniform_token_end(&self, token_definition: fn(char) -> bool) -> usize {
        for (index, character) in self.content.char_indices() {
            if !token_definition(character) {
                return index;
            }
        }

        self.content.len()
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // did we run out of tokens?
        if self.content.is_empty() {
            None
        }

        // is the next token a stretch of whitespace?
        else if self.content.get_char(0).expect("not empty").is_whitespace() {
            Some(self.cut_uniform_token(|c| c.is_whitespace()))
        }

        // is the next token numeric?
        else if self.content.get_char(0).expect("not empty").is_numeric() {
            Some(self.cut_uniform_token(|c| c.is_numeric()))
        }

        // is the next token alphabetic?
        else if self.content.get_char(0).expect("not empty").is_alphabetic() {
            Some(self.cut_uniform_token(|c| c.is_alphabetic()))
        }

        // is the next token a separator
        else if self.content.get_char(0).expect("not empty") == '.' {
            Some(self.cut_uniform_token(|c| c == '.'))
        }

        // the next token is a symbol
        else {
            Some(self.cut_uniform_token(|c| c != '.' && !c.is_alphanumeric() && !c.is_whitespace()))
        }
    }
}

#[test]
fn test_iteration() {
    let input = "...733.......289..262.....520..................161.462..........450.........................183.............................................";
    let mut lexer = Lexer::new(&input);

    assert_eq!(lexer.next().unwrap(), "...");
    assert_eq!(lexer.next().unwrap(), "733");
    assert_eq!(lexer.next().unwrap(), ".......");
    assert_eq!(lexer.next().unwrap(), "289");
    assert_eq!(lexer.next().unwrap(), "..");
    assert_eq!(lexer.next().unwrap(), "262");
    assert_eq!(lexer.next().unwrap(), ".....");
    assert_eq!(lexer.next().unwrap(), "520");
    assert_eq!(lexer.next().unwrap(), "..................");
    assert_eq!(lexer.next().unwrap(), "161");
    assert_eq!(lexer.next().unwrap(), ".");
    assert_eq!(lexer.next().unwrap(), "462");
    assert_eq!(lexer.next().unwrap(), "..........");
    assert_eq!(lexer.next().unwrap(), "450");
    assert_eq!(lexer.next().unwrap(), ".........................");
    assert_eq!(lexer.next().unwrap(), "183");
    assert_eq!(lexer.next().unwrap(), ".............................................");
}

#[test]
fn test_find_uniform_token_end() {
    let input = "...733.......289..262.....520..................161.462..........450.........................183.............................................";
    let mut lexer = Lexer::new(&input);

    fn numeric_token_definition(character: char) -> bool {
        character.is_numeric()
    }
    fn separator_token_definition(character: char) -> bool {
        character == '.'
    }

    let token1_end  = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token1_end);
    let token2_end  = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token2_end);
    let token3_end  = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token3_end);
    let token4_end  = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token4_end);
    let token5_end  = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token5_end);
    let token6_end  = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token6_end);
    let token7_end  = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token7_end);
    let token8_end  = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token8_end);
    let token9_end  = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token9_end);
    let token10_end = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token10_end);
    let token11_end = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token11_end);
    let token12_end = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token12_end);
    let token13_end = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token13_end);
    let token14_end = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token14_end);
    let token15_end = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token15_end);
    let token16_end = lexer.find_uniform_token_end(numeric_token_definition);
    lexer.cut_at(token16_end);
    let token17_end = lexer.find_uniform_token_end(separator_token_definition);
    lexer.cut_at(token17_end);

    assert_eq!(token1_end, 3);
    assert_eq!(token2_end, 3);
    assert_eq!(token3_end, 7);
    assert_eq!(token4_end, 3);
    assert_eq!(token5_end, 2);
    assert_eq!(token6_end, 3);
    assert_eq!(token7_end, 5);
    assert_eq!(token8_end, 3);
    assert_eq!(token9_end, 18);
    assert_eq!(token10_end, 3);
    assert_eq!(token11_end, 1);
    assert_eq!(token12_end, 3);
    assert_eq!(token13_end, 10);
    assert_eq!(token14_end, 3);
    assert_eq!(token15_end, 25);
    assert_eq!(token16_end, 3);
    assert_eq!(token17_end, 45);
}

#[test]
fn test_cut_at() {
    let input = "...733.......289..262.....520..................161.462..........450.........................183.............................................";
    let mut lexer = Lexer::new(&input);

    let token1 = lexer.cut_at(3);
    let token2 = lexer.cut_at(3);
    let token3 = lexer.cut_at(7);
    let token4 = lexer.cut_at(3);
    let token5 = lexer.cut_at(2);
    let token6 = lexer.cut_at(3);
    let token7 = lexer.cut_at(5);
    let token8 = lexer.cut_at(3);
    let token9 = lexer.cut_at(18);
    let token10 = lexer.cut_at(3);
    let token11 = lexer.cut_at(1);
    let token12 = lexer.cut_at(3);
    let token13 = lexer.cut_at(10);
    let token14 = lexer.cut_at(3);
    let token15 = lexer.cut_at(25);
    let token16 = lexer.cut_at(3);
    let token17 = lexer.cut_at(45);

    assert_eq!(token1, "...");
    assert_eq!(token2, "733");
    assert_eq!(token3, ".......");
    assert_eq!(token4, "289");
    assert_eq!(token5, "..");
    assert_eq!(token6, "262");
    assert_eq!(token7, ".....");
    assert_eq!(token8, "520");
    assert_eq!(token9, "..................");
    assert_eq!(token10, "161");
    assert_eq!(token11, ".");
    assert_eq!(token12, "462");
    assert_eq!(token13, "..........");
    assert_eq!(token14, "450");
    assert_eq!(token15, ".........................");
    assert_eq!(token16, "183");
    assert_eq!(token17, ".............................................");
}