use grid::Grid;

mod grid;

fn main() {
    let input = std::fs::read_to_string("day3.txt")
        .expect("Failed to open day 3 input");

    let grid: Grid<140, 140> = input.parse()
        .expect("failed to parse day 3 input");

    println!("{:#?}", grid);
}
