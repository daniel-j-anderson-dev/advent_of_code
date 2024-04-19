use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "../almanac.pest"]
pub struct AlmanacParser;
