use std::fs;

fn main() {
    let mut total: u64 = 0;

    // Read input file and split into lines
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    // Extract the last row, holding the operators, and remove it from the grid
    let operators = lines.last().unwrap_or(&"").split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    let lines = lines[..lines.len() - 1].to_vec();

    // Parse into a 2D grid, splitting each time there is a column of all spaces
    let grid: Vec<Vec<String>> = split_on_empty_columns(&lines);

    // Transpose the grid
    let transposed_grid: Vec<Vec<String>> = transpose(&grid);

    // Trim any leftover whitespace
    let cleaned_grid: Vec<Vec<String>> = transposed_grid.iter()
        .map(|row| row.iter().map(|s| s.trim().to_string()).collect())
        .collect();

    // For each problem, determine if it's an addition or multiplication problem and solve accordingly, then add the result to total
    for i in 0..operators.len() {
        let op = &operators[i];
        match op.as_str() {
            "+" => total += add(&cleaned_grid[i]),
            "*" => total += product(&cleaned_grid[i]),
            _ => eprintln!("Invalid operator: {}", op)
        }
    }

    println!("{}", total);
}

// Calculate the sum of every digit in the row, ignoring the operator if present
fn add(row: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    for c in row {
        sum += c.parse().unwrap_or(0);
    }

    sum
}

// Calculate the product of every digit in the row, ignoring the operator if present
fn product(row: &Vec<String>) -> u64 {
    let mut product: u64 = 1;
    for c in row {
        product *= c.parse().unwrap_or(1);
    }

    product
}

// Split the lines only on columns with all spaces, and return the exact substrings for each column, without removing important whitespace
fn split_on_empty_columns(lines: &[&str]) -> Vec<Vec<String>> {
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pad for column analysis only
    let padded: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    // Find columns that are spaces in every line
    let empty_cols: Vec<bool> = (0..width)
        .map(|col| padded.iter().all(|line| line[col] == ' '))
        .collect();

    // Build ranges of non-empty columns
    let mut ranges = Vec::new();
    let mut start = None;

    for i in 0..width {
        if !empty_cols[i] {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(s) = start {
            ranges.push((s, i));
            start = None;
        }
    }

    if let Some(s) = start {
        ranges.push((s, width));
    }

    // Extract exact substrings
    lines.iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            ranges.iter()
                .filter_map(|&(s, e)| {
                    if s >= chars.len() {
                        return None;
                    }

                    let end = e.min(chars.len());

                    Some(chars[s..end].iter().collect::<String>())
                })
                .collect()
        })
        .collect()
}

// Transpose the grid of problems, so we can read each number
fn transpose(data: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<Vec<String>> = Vec::new();

    // For each column in the input
    for i in 0..data[0].len() {
        // Gather the associated strings
        let problem: Vec<String> = data.iter()
            .map(|line| line[i].clone())
            .collect();

        // Convert to characters
        let problem_chars: Vec<Vec<char>> = problem.iter()
            .map(|s| s.chars().collect())
            .collect();

        // Transpose
        let transposed_chars: Vec<Vec<char>> = (0..problem_chars[0].len())
            .map(|col_idx| {
                problem_chars.iter().filter_map(|line| line.get(col_idx)).copied().collect()
            })
            .collect();

        // Convert back to strings
        let transposed_strings: Vec<String> = transposed_chars.iter()
            .map(|chars| chars.iter().collect::<String>())
            .collect();

        // Append to result
        result.push(transposed_strings);
    }

    result
}
