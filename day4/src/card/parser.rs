use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use pest::Parser;
use pest_derive::Parser;

use crate::card::Card;

#[derive(Parser)]
#[grammar = "../card.pest"]
pub struct CardParser;

#[derive(Debug, PartialEq)]
pub struct Cards(pub Vec<Card>);
impl FromStr for Cards {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = CardParser::parse(Rule::File, s)?;
        return Ok(Self(
            cards.filter_map(|card| card.try_into().ok()).collect(),
        ));
    }
}
impl FromIterator<Card> for Cards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        return Self(iter.into_iter().collect());
    }
}
impl Deref for Cards {
    type Target = Vec<Card>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for Cards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

#[test]
fn f() {
    use crate::INPUT;
    let from_scratch = INPUT
        .lines()
        .filter_map(|line| line.parse::<Card>().ok())
        .collect::<Vec<_>>();
    let from_pest = INPUT.parse::<Cards>().unwrap().0;
    assert_eq!(from_scratch, from_pest);
}
