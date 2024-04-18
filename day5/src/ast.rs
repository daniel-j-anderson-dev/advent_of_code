use pest::iterators::Pair;

use crate::parser::Rule;

pub enum Node {
    Seeds(Vec<usize>),
    Map {
        name: String,
        values: Vec<(usize, usize, usize)>,
    },
}
// impl TryFrom<Pair<'_, Rule>> for Node {
//     type Error = Box<dyn std::error::Error>;
//     fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
//         let node = match value.as_rule() {
//             Rule::Seeds => {
//                 let inner_rules = value.into_inner();
//                 let values = inner_rules.map(|pair| pair.as_str().parse().unwrap()).collect();
//                 Node::Seeds(values)
//             },
//             Rule::Map => {
//                 let inner_rules = value.into_inner();
                
//             },
//             Rule::EOI
//             | Rule::File
//             | Rule::MapEntry
//             | Rule::Name
//             | Rule::Number
//             | Rule::WHITESPACE => Err("")?,
//         };

//         return Ok(node);
//     }
// }