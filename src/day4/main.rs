use std::fs;

fn find_desired_pattern(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut validPatterns: i32 = 0;

    for row in 0..rows {
        for column in 0..cols {
            if grid[row][column] == 'A' {
                if row > 1 && row < rows - 1 && column > 1 && column < cols - 1 {
                    let char1_diag1 = grid[row + 1][column - 1];
                    let char2_diag1 = grid[row - 1][column + 1];

                    let char1_diag2 = grid[row + 1][column + 1];
                    let char2_diag2 = grid[row - 1][column - 1];

                    let accepted_pattern_listen = ["MAS", "SAM"];

                    let diag_1 = String::from_iter([char1_diag1, 'A', char2_diag1]);
                    let diag_2 = String::from_iter([char1_diag2, 'A', char2_diag2]);

                    if (diag_1 == accepted_pattern_listen[0]
                        || diag_1 == accepted_pattern_listen[1])
                        && (diag_2 == accepted_pattern_listen[0]
                            || diag_2 == accepted_pattern_listen[1])
                    {
                        validPatterns += 1
                    }
                }
            }
        }
    }

    return validPatterns;
}

pub fn main() {
    // Read the input grid
    let input = fs::read_to_string("./src/day4/input.txt").expect("Failed to read the input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result = find_desired_pattern(grid);

    println!("result: {}", result);

    // Count all X-MAS patterns
}
