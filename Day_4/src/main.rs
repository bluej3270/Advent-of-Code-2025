use std::fs;

fn main() {
    let mut total: u64 = 0;

    // Read input file and split into lines
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    // Convert into a 2D grid
    let mut grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    // Track the total rolls removed on the previous iteration, and loop until no more paper rolls can be removed
    let mut prev_total: u64 = 1; // Start at 1 to enter the loop at least once

    while prev_total > 0 {
        // Count and remove all possible roles, then update values for next iteration
        let (removed, newgrid) = remove_paper(&grid);
        total += removed;
        prev_total = removed;
        grid = newgrid;
    }

    println!("{}", total);

}


// Given a grid, and a position (row, col), return a vector of the 8 adjacent characters (or fewer if on an edge)
fn get_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut neighbors = Vec::new();

    // Check all 8 adjacent positions
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue; // Skip the center cell
            }

            let new_row = (row as i32 + dr) as usize;
            let new_col = (col as i32 + dc) as usize;

            // Boundary check
            if new_row < rows && new_col < cols {
                neighbors.push(grid[new_row][new_col]);
            }
        }
    }

    neighbors
}

// Loop through the grid and count all paper rolls that could be removed. Return that total, and an updated grid with those papers removed
fn remove_paper(grid: &Vec<Vec<char>>) -> (u64, Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut total: u64 = 0;
    let mut newgrid: Vec<Vec<char>> = grid.clone();

    // Loop through each cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Check if the cell is a roll of paper
            if grid[row][col] == '@' {
                // Get the neighbors of this cell
                let neighbors = get_neighbors(&grid, row, col);

                // If fewer than 4 of the neighbors are a roll of paper (@), add one to total and mark that roll as removed in newgrid
                if neighbors.iter().filter(|&&c| c == '@').count() < 4 {
                    newgrid[row][col] = '.';
                    total += 1;
                }
            }

        }
    }

    (total, newgrid)
}