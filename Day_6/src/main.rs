use std::fs;

fn main() {
    let mut total: u64 = 0;

    // Read input file and split into lines
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    // Convert into a 2D grid, splitting each time there are one or more spaces
    let grid: Vec<Vec<String>> = lines
        .iter()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();


    // Transpose grid so each problem is in its own row
    let mut transposed_grid: Vec<Vec<String>> = Vec::new();
    for i in 0..grid[0].len() {
        let mut row: Vec<String> = Vec::new();
        for j in 0..grid.len() {
            row.push(grid[j][i].clone());
        }
        transposed_grid.push(row);
    }

    // For each problem, determine if it's an addition or multiplication problem and solve accordingly, then add the result to total
    for row in transposed_grid {
        if let Some(op) = row.last() {
            match op.as_str() {
                "+" => total += add(&row) as u64,
                "*" => total += product(&row) as u64,
                _ => eprintln!("Invalid operator: {}", op)
            }
        }
    }

    println!("{}", total);

}

// Calculate the sum of every digit in the row, ignoring the operator
fn add(row: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    for c in row {
        sum += c.parse().unwrap_or(0);
    }

    sum
}

// Calculate the product of every digit in the row, ignoring the operator
fn product(row: &Vec<String>) -> u64 {
    let mut product: u64 = 1;
    for c in row {
        product *= c.parse().unwrap_or(1);
    }

    product
}
