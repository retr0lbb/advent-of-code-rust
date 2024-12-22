use std::fs;

fn find_x_mas(grid: &[Vec<char>], rows: usize, cols: usize) -> usize {
    let mut count = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if is_x_mas(grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    // Define positions for the X-MAS structure
    let top_left = grid.get(row.wrapping_sub(1)).and_then(|r| r.get(col.wrapping_sub(1)));
    let top_right = grid.get(row.wrapping_sub(1)).and_then(|r| r.get(col + 1));
    let center = grid.get(row).and_then(|r| r.get(col));
    let bottom_left = grid.get(row + 1).and_then(|r| r.get(col.wrapping_sub(1)));
    let bottom_right = grid.get(row + 1).and_then(|r| r.get(col + 1));

    // Check if all positions form an X-MAS pattern
    let top_valid = matches!((top_left, top_right), (Some('M'), Some('S')) | (Some('S'), Some('M')));
    let bottom_valid = matches!((bottom_left, bottom_right), (Some('M'), Some('S')) | (Some('S'), Some('M')));

    top_valid && bottom_valid && center == Some(&'A')
}

pub fn main() {
    // Read the input grid
    let input = fs::read_to_string("./src/day4/input.txt").expect("Failed to read the input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    // Count all X-MAS patterns
    let count = find_x_mas(&grid, rows, cols);

    println!("X-MAS appears {} times.", count);
}
