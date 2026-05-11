use std::fs;

fn main() {
    let mut total: u64 = 0;

    // Read and parse input file
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut available_ids: Vec<u64> = Vec::new();

    let mut in_ranges = true;
    for line in lines {
        if line.trim().is_empty() {
            in_ranges = false;
            continue;
        }

        if in_ranges {
            ranges.push({
                // Convert range string "start-end" into a tuple (start, end)
                let parts: Vec<&str> = line.split('-').collect();
                if parts.len() == 2 {
                    let start = parts[0].parse().ok().expect("Invalid range start");
                    let end = parts[1].parse().ok().expect("Invalid range end");
                    (start, end)
                } else {
                    panic!("Invalid range format: {}", line);
                }
            });
        } else {
            // Convert id's from string to integer, and store
            available_ids.push(line.parse::<u64>().unwrap());
        }
    }

    // Part 1: Check each available ID against the ranges, and count how many are valid
    for id in available_ids {
        if is_in_ranges(id, &ranges) {
            total += 1;
        }
    }

    println!("Part 1: {}", total);

    // Part 2: Merge the ranges to remove any overlap, then count how many total ingredient ID's are covered by the merged ranges
    ranges = merge_ranges(&ranges);
    total = 0;

    for range in ranges {
        total += (range.1 - range.0) + 1; // Add the number of IDs covered by this range (inclusive)
    }

    println!("Part 2: {}", total);
}

// Test if a given value is in the supplied vector of ranges formatted "start-end"
fn is_in_ranges(value: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for &(start, end) in ranges {
        if value >= start && value <= end {
            return true;
        }
    }
    false
}

// Merge ranges to remove any overlap
fn merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut ranges = ranges.clone();

    // Sort by start
    ranges.sort_by_key(|&(s, _)| s);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];

    // Loop through each range. If it overlaps with the current range, merge them. Otherwise, add the current range to the merged list and start a new current range.
    for &(start, end) in &ranges[1..] {
        if start <= current.1 {
            // Overlap, merge
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
}