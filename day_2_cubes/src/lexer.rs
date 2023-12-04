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

    /// returns the start/end of a alphabetic token representing a digit
    fn find_digit_str_token(&self) -> Option<usize> {
        if !self.content[0].is_alphabetic() {
            return None;
        }

        let alpha_end = self.find_token_end(|character| character.is_alphabetic());
        let alpha_token = &self.content[..alpha_end];

        for i in 0..alpha_end {
            let slice = &alpha_token[i..];
    
            if  slice.starts_with(&['r','e','d']) {
                if i != 0 {
                    return Some(i);
                } else {
                    return Some(3);
                }
            }
            if  slice.starts_with(&['g','r','e','e','n']) {
                if i != 0 {
                    return Some(i);
                } else {
                    return Some(5);
                }
            }
            if  slice.starts_with(&['G','a','m','e']) ||
                slice.starts_with(&['b','l','u','e'])
            {
                if i != 0 {
                    return Some(i);
                } else {
                    return Some(4);
                }
            }
        }
    
        None
    }

    /// This function truncates the left most token (ignoring whitespace) and returns it
    /// 
    /// 
    /// # Parameters 
    ///  - token_definition: a non environment capturing function that represents which characters are valid for a token
    fn cut_token(&mut self, token_definition: fn(char) -> bool) -> &'a [char] {
        self.cut(self.find_token_end(token_definition))
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        // did we run out of tokens?
        if self.content.is_empty() {
            None
        }

        // is the token a stretch of whitespace?
        else if self.content[0].is_whitespace() {
            self.cut_token(|character| character.is_whitespace());
            self.next_token()
        }

        // is the token a number. only do single digits
        else if self.content[0].is_numeric() {
            Some(self.cut_token(|character| character.is_numeric()))
        }

        // is the token a string
        else if self.content[0].is_alphabetic() {
            if let Some(token_end) = self.find_digit_str_token() {
                Some(self.cut(token_end))
            }
            else {
                self.cut_token(|character| character.is_alphabetic());
                self.next_token()
            }
        }

        else {
            Some(self.cut(1))
        }
    }
}
impl Into<Vec<String>> for Lexer<'_> {
    fn into(self) -> Vec<String> {
        self.map(|token| {
            token.iter().collect()
        })
        .collect()
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[test]
fn alpha_ending_in_num() {
    let input = "xyzone
abctwo
abctwo4";

    for line in input.lines() {
        let line_chars = line.chars().collect::<Vec<_>>();
        let tokens: Vec<String> = Lexer::new(&line_chars).into();
        
        print!("{}\n[", line);
        for (i, token) in tokens.iter().enumerate() {
            print!("{}{}", token, if i == tokens.len() - 1 {""} else {", "});
        }
        println!("]\n");
    }
}

#[test]
fn test() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    for line in input.lines() {
        let line_chars = line.chars().collect::<Vec<_>>();
        let tokens: Vec<String> = Lexer::new(&line_chars).into();

        print!("{}\n[", line);
        for (i, token) in tokens.iter().enumerate() {
            print!("{}{}", token, if i == tokens.len() - 1 {""} else {", "});
        }
        println!("]\n");
    }
}

#[test]
fn number_tokens() {
    let input = "oneone
twotwo
threethree
fourfour
fivefive
sixsix
sevenseven
eighteight
ninenine".chars().collect::<Vec<_>>();

    let tokens: Vec<String> = Lexer::new(&input).into();

    print!("{}\n[", input.iter().collect::<String>());
    for (i, token) in tokens.iter().enumerate() {
        print!("{}{}", token, if i == tokens.len() - 1 {""} else {", "});
    }
    println!("]\n");

    assert_eq!("one",   tokens[ 0]);
    assert_eq!("one",   tokens[ 1]);
    assert_eq!("two",   tokens[ 2]);
    assert_eq!("two",   tokens[ 3]);
    assert_eq!("three", tokens[ 4]);
    assert_eq!("three", tokens[ 5]);
    assert_eq!("four",  tokens[ 6]);
    assert_eq!("four",  tokens[ 7]);
    assert_eq!("five",  tokens[ 8]);
    assert_eq!("five",  tokens[ 9]);
    assert_eq!("six",   tokens[10]);
    assert_eq!("six",   tokens[11]);
    assert_eq!("seven", tokens[12]);
    assert_eq!("seven", tokens[13]);
    assert_eq!("eight", tokens[14]);
    assert_eq!("eight", tokens[15]);
    assert_eq!("nine",  tokens[16]);
    assert_eq!("nine",  tokens[17]);
}
