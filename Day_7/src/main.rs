use std::fs;

fn main() {
    let mut total: u64 = 0;

    // Read input file
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    // Convert lines to full strings (instead of string slices) to allow easier manipulation
    let mut lines: Vec<String> = lines.into_iter().map(|line| line.to_string()).collect();

    // Loop through the first line to find the starting point "S"
    let start = lines[0].chars()
        .position(|c| c == 'S')
        .expect("Starting point 'S' not found");

    // Place a "tachyon" ("|") below the starting point
    lines[1] = lines[1].chars()
        .enumerate()
        .map(|(i, c)| if i == start { '|' } else { c })
        .collect::<String>();

    // Starting with index 1, loop through the lines
    for i in 1..lines.len() - 1 {
        // Loop through the characters in the line.
        for j in 0..lines[i].len() {
            // If there is a "tachyon", look at the line below.
            if lines[i].chars().nth(j) == Some('|') {
                let mut nextline = lines[i+1].chars().collect::<Vec<char>>();
                // If there is a splitter, place a "tachyon" on either side of the splitter.
                if nextline[j] == '^' {
                    nextline[j-1] = '|';
                    nextline[j+1] = '|';

                    total += 1; // Increment total for each splitter encountered
                }
                // Otherwise, place a "tachyon" directly below.
                else {
                    nextline[j] = '|';
                }
                lines[i+1] = nextline.iter().collect::<String>();
            }
        }
    }

    println!("{}", total);
}
