use std::fs;

fn main() {
    let mut total: u64 = 0;

    // Read and parse input file
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    let mut ranges: Vec<String> = Vec::new();
    let mut available_ids: Vec<u64> = Vec::new();

    let mut in_ranges = true;
    for line in lines {
        if line.trim().is_empty() {
            in_ranges = false;
            continue;
        }

        if in_ranges {
            ranges.push(line.to_string());
        } else {
            available_ids.push(line.parse::<u64>().unwrap());
        }
    }

    // Check each available ID against the ranges, and count how many are valid
    for id in available_ids {
        if is_in_ranges(id, &ranges) {
            total += 1;
        }
    }

    println!("Total: {}", total);
}


// Test if a given value is in the supplied vector of ranges formatted "start-end"
fn is_in_ranges(value: u64, ranges: &Vec<String>) -> bool {
    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            println!("Invalid range: {}", range); // Notify if there are any improperly formatted ranges
            continue; // Skip invalid ranges
        }
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        if value >= start && value <= end {
            return true;
        }
    }
    false
}