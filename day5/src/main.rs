use pest::Parser;

pub mod ast;
pub mod parser;
use crate::{
    ast::Almanac,
    parser::{AlmanacParser, Rule},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");
}

#[test]
fn print_parse_output() {
    let file = AlmanacParser::parse(Rule::File, INPUT).unwrap();
    for pair in file {
        println!("{:#?}", pair);
    }
}

#[test]
fn ast() {
    let almanac = INPUT.parse::<Almanac>().unwrap();
    for node in almanac.iter() {
        println!("{}", node);
    }
}
