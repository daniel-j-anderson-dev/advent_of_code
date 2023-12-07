// credit to carlomilanesi!: https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11
use std::ops::{
    Bound,
    RangeBounds,
};

pub trait StringUtilities {
    fn substring(&self, start: usize, length: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
    fn get_char(&self, index: usize) -> Option<char>;
}
impl StringUtilities for str {
    fn substring(&self, start: usize, length: usize) -> &str {
        let mut character_index = 0;
        let mut byte_start = 0;
        let mut characters = self.chars();

        while character_index != start {
            match characters.next() {
                Some(character) => {
                    character_index += 1;
                    byte_start += character.len_utf8();
                },
                None => break,
            }    
        }

        let mut character_index = 0;
        let mut byte_end = byte_start;

        while character_index != length {
            match characters.next() {
                Some(character) => {
                    character_index += 1;
                    byte_end += character.len_utf8();
                },
                None => break,
            }
        }

        &self[byte_start..byte_end]
    }

    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) |
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };

        let length = match range.end_bound() {
            Bound::Included(bound) => (*bound + 1) - start,
            Bound::Excluded(bound) => *bound - start,
            Bound::Unbounded => self.len() - start,
        };

        self.substring(start, length)
    }

    fn get_char(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
}
