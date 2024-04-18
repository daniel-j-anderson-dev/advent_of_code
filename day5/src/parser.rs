use pest::Parser;
use pest_derive::Parser;

use crate::INPUT;

#[derive(Debug, Parser)]
#[grammar = "../almanac.pest"]
pub struct AlmanacParser;

#[test]
fn f() {
    let file = AlmanacParser::parse(Rule::File, INPUT);

    if let Err(ref e) = file {
        println!("{}\n{:#?}", e, e);
    }

    let file = file.unwrap();
    for pair in file {
        println!("{:#?}", pair);
    }
}