pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}
impl Card {
    fn get_id(slice: &str) -> u32 {
        slice
            .chars()
            .skip_while(|ch| !ch.is_numeric())
            .take_while(|ch| ch.is_numeric())
            .collect::<String>()
            .parse()
            .expect("No card id")
    }
    fn get_winning_numbers(slice: &str) -> Vec<u32> {
        slice
            .split_whitespace()
            .take_while(|number| *number != "|")
            .filter_map(|number| number.parse().ok())
            .collect()
    }
    fn get_numbers(slice: &str) -> Vec<u32> {
        slice
            .split_whitespace()
            .filter_map(|number| number.parse().ok())
            .collect()
    }
    
    pub fn points(&self) -> u32 {
        let winner_count = self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as u32;

        if winner_count == 0 {
            0
        } else {
            u32::pow(2, winner_count - 1)
        }
    }
}
impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let colon_index = value.find(':').expect("no ':' delimiter");
        let pipe_index = value.find('|').expect("no '|' delimiter");

        let id = Card::get_id(value);
        let winning_numbers = Card::get_winning_numbers(&value[colon_index..]);
        let numbers = Card::get_numbers(&value[pipe_index..]);

        Self { id, winning_numbers, numbers }
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {:>3}: ", self.id)?;
        for (index, number) in self.winning_numbers.iter().enumerate() {
            write!(f, "{:>2}{}", number, if index == self.winning_numbers.len() - 1 {" | "} else {" "})?;
        }
        for (index, number) in self.numbers.iter().enumerate() {
            write!(f, "{:>2}{}", number, if index == self.numbers.len() - 1 {""} else {" "})?;
        }
        Ok(())
    }
}


#[test]
fn test_get_id() {
    for (line_index, line) in crate::INPUT.lines().enumerate() {
        assert_eq!(Card::get_id(line), line_index as u32 + 1);
    }
}
#[test]
fn card_from_str() {
    for line in crate::INPUT.lines() {
        let card = Card::from(line);
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
        assert_eq!(card_points, Card::from(line).points());
    }
}