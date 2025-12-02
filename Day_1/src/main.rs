use std::fs;

fn main() {
    let mut position = 50;
    let mut zeros = 0;

    // Read from file and split each input into a Vector (array)
    let input = fs::read_to_string("input.txt").expect("File not found");
    let moves: Vec<&str> = input.lines().collect();

    // Loop through each input.
    for index in 0..moves.len() {
        // Parse the input string into direction and distance
        let (direction, distance) = parse_move(moves[index]).unwrap();

        // Parse distance from string to int
        let distance: i32 = distance.parse().unwrap();

        // Move the dial (Part 1)
        // position = turn(direction, distance, position);

        // Move the dial (Part 2)
        (position, zeros) = turn(direction, distance, position, zeros);

        // Count if the dial lands on a zero (Part 1).
        // if position == 0 {zeros += 1;}

        println!("Position: {}, Distance: {}", position, distance);
    }

    // Print number of zeros
    println!("Zeros: {}", zeros);
}

fn parse_move(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    chars.next().map(|c| (c, chars.as_str()))
}

// For Part 1
// fn turn (direction: char, distance: i32, pos: i32) -> i32 {
//     let new_pos = {
//         match direction {
//             'L' => pos - distance,
//             'R' => pos + distance,
//             _ => pos,
//         }
//     };
//
//     new_pos.rem_euclid(100)
// }

fn turn (direction: char, distance: i32, pos: i32, zeros: i32) -> (i32, i32) {
    let mut new_zeros = zeros;
    let mut new_pos = pos;

    if direction == 'L' {
        for _i in 0..distance {
            new_pos -= 1;

            // Roll over
            if new_pos == -1 {new_pos = 99}

            // Count Zeros
            if new_pos == 0 {new_zeros += 1;}
        }
    }
    else if direction == 'R' {
        for _i in 0..distance {
            new_pos += 1;

            // Roll over
            if new_pos == 100 {new_pos = 0;}

            // Count Zeros
            if new_pos == 0 {new_zeros += 1;}
        }
    }

    (new_pos, new_zeros)
}