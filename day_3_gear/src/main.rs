use grid::{Grid, TokenKind};

mod grid;

fn main() {
    let input = std::fs::read_to_string("day3.txt")
        .expect("Failed to open day 3 input");

    let grid: Grid<140, 140> = input.parse()
        .expect("failed to parse day 3 input");
    let mut part_numbers = Vec::new();

    let number_started = false;

    for (row_index, row) in grid.rows().iter().enumerate() {
        for (col_index, token) in row.iter().enumerate() {
            
            if token.kind().is_number()  {
                
                if grid.is_token_a_part_number(row_index, col_index) {
                    part_numbers.push(token);
                }
            }
        }
    }

    println!("{:#?}", part_numbers);
}

