use std::{fmt::Display, ops::Deref, str::FromStr};

use pest::{iterators::Pair, Parser};

use crate::parser::{AlmanacParser, Rule};

pub struct Almanac(Vec<Group>);
impl FromStr for Almanac {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file = AlmanacParser::parse(Rule::File, s)?;

        let nodes = file
            .filter_map(|pair| {
                if let Rule::EOI = pair.as_rule() {
                    None
                } else {
                    Group::try_from(pair).ok()
                }
            })
            .collect();

        return Ok(Almanac(nodes));
    }
}
impl AsRef<[Group]> for Almanac {
    fn as_ref(&self) -> &[Group] {
        return &self.0;
    }
}
impl Deref for Almanac {
    type Target = [Group];
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

pub enum Group {
    Seeds(Vec<usize>),
    Map {
        name: String,
        values: Vec<(usize, usize, usize)>,
    },
}
impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Group::Seeds(values) => {
                write!(f, "seeds: ")?;
                for value in values {
                    write!(f, "{} ", value)?;
                }
                write!(f, "\n")?;
            }
            Group::Map { name, values } => {
                write!(f, "{}:\n", name)?;
                for (a, b, c) in values {
                    write!(f, "{} {} {}\n", a, b, c)?;
                }
            }
        };
        return Ok(());
    }
}
impl TryFrom<Pair<'_, Rule>> for Group {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let node = match value.as_rule() {
            Rule::Group => {
                let mut inner_rules = value.into_inner();

                let name = inner_rules.next().expect("Every Rule::Group has a name first").as_str();

                match name {
                    "seeds" => Group::Seeds(
                        inner_rules
                            .map(|number| number.as_str().parse().expect("Only numeric chars"))
                            .collect(),
                    ),
                    _ => {
                        let name = name.to_owned();
                        let mut values = Vec::new();
                        loop {
                            if let Some(inner_number_rule) = inner_rules.next() {
                                let value = (
                                    inner_number_rule
                                        .as_str()
                                        .parse()
                                        .expect("only numeric chars"),
                                    inner_rules
                                        .next()
                                        .expect("Groups not named seed contain one or more triplets")
                                        .as_str()
                                        .parse()
                                        .expect("only numeric chars"),
                                    inner_rules
                                        .next()
                                        .expect("Groups not named seed contain one or more triplets")
                                        .as_str()
                                        .parse()
                                        .expect("only numeric chars"),
                                );

                                values.push(value);
                            } else {
                                break;
                            }
                        }
                        Group::Map { name, values }
                    }
                }
            }
            rule => Err(format!(
                "the only valid group is Rule::Group. Rule found: {:?}",
                rule
            )
            .as_str())?,
        };
        return Ok(node);
    }
}
