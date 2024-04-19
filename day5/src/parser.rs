use pest::Parser;

use pest_derive::Parser;

use crate::{ast::Almanac, INPUT};

#[derive(Debug, Parser)]
#[grammar = "../almanac.pest"]
pub struct AlmanacParser;
