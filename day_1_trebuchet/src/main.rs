mod lexer;
use lexer::Lexer;

fn main() {
    let input = std::fs::read_to_string("day1.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let digits = decode_digits(line);

        let calibration_value = decode_calibration_value(&digits);

        sum += calibration_value;
    }

    println!("{}", sum);
}

fn decode_digits(line: &str) -> Vec<usize> {
    let line: Vec<char> = line.chars().collect();
    let tokens: Vec<String> = Lexer::new(&line).into();

    let digits = tokens
        .iter()
        .filter_map(|token| token_to_digit(token))
        .collect();

    digits
}

fn decode_calibration_value(digits: &[usize]) -> usize {
    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(&0);

    let calibration_value = format!("{}{}", first, last)
        .parse()
        .expect("first and last are digits");

    calibration_value
}

fn token_to_digit(token: &str) -> Option<usize> {
    match token {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => match token.parse::<usize>() {
            Ok(digit) => Some(digit),
            Err(_) => None,
        },
    }
}
