use std::fs;

fn main() {
    let mut total:u64 = 0;

    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines {
        total += solve_battery(line);
    }

    println!("total: {}", total);
}

// Find the largest 2-digit number from a sequence of numbers, maintaining order (Part 1)
// fn solve_battery(joltage: &str) -> u32 {
//     let joltage = joltage.trim().chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<u32>>();
//
//     let mut max  = 0;
//     let mut next = joltage.len()-1;
//
//     for i in 0..joltage.len()-1 {
//         max = if joltage[i] > joltage[max] {i} else {max}
//     }
//
//     for i in max+1..joltage.len() {
//         next = if joltage[i] > joltage[next] {i} else {next}
//     }
//
//     (joltage[max]*10) + joltage[next]
// }

// Find the largest 12-digit number from a sequence of numbers, maintaining order (Part 2)
fn solve_battery(joltage: &str) -> u64 {
    let joltage = joltage.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let mut result:u64 = 0;
    let mut last_index:usize = 0;

    // Find each digit, multiply it to place it in the correct spot, add it to result
    for i in (0..12).rev() {
        last_index = find_next_digit(&joltage, last_index, i);
        result = result + ((joltage[last_index] as u64) * 10_u64.pow(i));
        last_index += 1; //Always start looking at the next index, can't select one battery twice
    }

    result
}

// Find the largest digit from a battery bank, starting at last index, and leaving enough digits left to finish selecting
fn find_next_digit(joltage: &Vec<u32>, last_index: usize, digit: u32) -> usize {
    let mut max = last_index;
    for i in last_index..joltage.len()-(digit as usize) {
        max = if joltage[i] > joltage[max] {i} else {max}
    }

    max
}