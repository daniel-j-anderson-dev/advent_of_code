use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use pest::{iterators::Pair, Parser};

use self::parser::{CardParser, Rule};

pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}
impl Card {
    pub fn id(&self) -> usize {
        return self.id;
    }
    pub fn winner_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as usize
    }
    pub fn points(&self) -> usize {
        let winner_count = self.winner_count();

        if winner_count == 0 {
            0
        } else {
            usize::pow(2, winner_count as u32 - 1)
        }
    }
}
impl TryFrom<Pair<'_, Rule>> for Card {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: pest::iterators::Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let card = match value.as_rule() {
            Rule::Card => {
                let mut inner_rules = value.into_inner();

                let id = inner_rules.next().unwrap().as_str().parse().unwrap();
                let winning_numbers = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                let numbers = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                Card {
                    id,
                    winning_numbers,
                    numbers,
                }
            }
            rule => Err(format!(
                "Can only create Card from Rule::Card; rule found: {:?}",
                rule
            ))?,
        };

        return Ok(card);
    }
}
impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_index = s.find(':').ok_or("no ':' delimiter")?;
        let pipe_index = s.find('|').ok_or("no '|' delimiter")?;

        let id = s
            .chars()
            .skip_while(|ch| !ch.is_numeric())
            .take_while(|ch| ch.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .expect("Only take numeric chars");

        let winning_numbers = s[colon_index..pipe_index]
            .split_whitespace()
            .filter_map(|sub_slice| sub_slice.parse().ok())
            .collect();

        let numbers = s[pipe_index..]
            .split_whitespace()
            .filter_map(|sub_slice| sub_slice.parse().ok())
            .collect();

        Ok(Self {
            id,
            winning_numbers,
            numbers,
        })
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {:>3}: ", self.id)?;
        for (index, number) in self.winning_numbers.iter().enumerate() {
            write!(
                f,
                "{:>2}{}",
                number,
                if index == self.winning_numbers.len() - 1 {
                    " | "
                } else {
                    " "
                }
            )?;
        }
        for (index, number) in self.numbers.iter().enumerate() {
            write!(
                f,
                "{:>2}{}",
                number,
                if index == self.numbers.len() - 1 {
                    ""
                } else {
                    " "
                }
            )?;
        }
        Ok(())
    }
}

#[test]
fn card_from_str() {
    for line in crate::INPUT.lines() {
        let card = line.parse::<Card>().unwrap();
        assert_eq!(card.to_string(), line);
    }
}

#[test]
fn test_points() {
    let input = "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card   2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card   3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card   4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card   5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card   6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let points = [8, 2, 2, 1, 0, 0];

    for (line, card_points) in input.lines().zip(points) {
        assert_eq!(card_points, line.parse::<Card>().unwrap().points());
    }
}
