mod lexer;
use lexer::Lexer;
use std::fs;

fn main() {
    let input = fs::read_to_string("E:/src/rust/advent_of_code/day_1_trebuchet/input.txt").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        sum += decode_line(line);
    }
    
    println!("{}", sum);
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
            Ok(num) => Some(num),
            Err(_) => {
                None
            },
        }
    }
}

fn decode_line(line: &str) -> usize {
    let line_chars = line.chars().collect::<Vec<_>>();
    let tokens: Vec<String> = Lexer::new(&line_chars).into();
    
    let digits = tokens.iter()
        .map(|token| token_to_digit(token))
        .filter(|o| o.is_some())
        .map(|o| o.expect("filtered for only Some"))
        .collect::<Vec<_>>();

    let first = digits.iter().nth(0)
        .expect("line should not be empty");
    let last = digits.iter().last()
        .expect("line should not be empty");
    
    let calibration_value = format!("{}{}", first, last)
        .parse()
        .expect("only digit chars");

    println!("line: {}\ntokens: {:?}\ndigits: {:?}\ncalibration value: {}\n", line, tokens, digits, calibration_value);

    calibration_value
} 

#[test]
fn test() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let mut sum = 0;

    for line in input.lines() {
        let calibration_value = decode_line(line);

        sum += calibration_value;

        println!("{}", calibration_value)
    }

    println!("{}", sum);
    assert!(sum == 142);   
}