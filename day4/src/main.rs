mod card;
use card::{parser::Cards, Card};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deck = INPUT
        .parse::<Cards>()?
        .iter()
        .map(|card| (card.id(), card.winner_count()))
        .collect::<Vec<_>>();

    let mut i = 0;
    loop {
        i += 1;
        if i >= deck.len() {
            break;
        }
    }

    return Ok(());
}

#[test]
fn f() {
    let from_scratch = INPUT
        .lines()
        .filter_map(|line| line.parse::<Card>().ok())
        .collect::<Vec<_>>();
    let from_pest = INPUT.parse::<Cards>().unwrap().0;
    assert_eq!(from_scratch, from_pest);
}

/*
    for dcard in deck
        hand.push(dcard)
        for hcard in hand
            for (hcard.get_win_count())
                hand.push(deck[dcard_index + i + 1].clone())

    stop condition -> any card that has win_count == 0
*/
