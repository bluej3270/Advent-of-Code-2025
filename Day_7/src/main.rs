use std::fs;

fn main() {
    let total: u64;

    // Read input file
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    // Convert lines to vectors of characters to allow easier manipulation
    let mut lines: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();

    // Loop through the first line to find the starting point "S"
    let start = lines[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("Starting point 'S' not found");

    // Place a "tachyon" ("|") at the starting point
    lines[0] = lines[0].iter()
        .enumerate()
        .map(|(i, &c)| if i == start { '|' } else { c })
        .collect();

    total = simulate_particle(lines, start);

    println!("{}", total);
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Ray {
    row: usize,
    col: usize,
    weight: usize // The weight represents how many possible worlds had a ray in this position
}

fn simulate_particle(manifold: Vec<Vec<char>>, start: usize) -> u64 {
    // Start with one ray at the starting point, with weight one (because there is one possible world at the start).
    // We will observe the rays in discrete time steps, so a ray can only be in one place at a time. Each ray will either split
    // or move at each time step. The weight of a ray represents how many possible worlds had a ray in that position.
    // Credit to u/SupportPowerful6174 for the inspiration behind this algorithm: https://www.reddit.com/r/adventofcode/comments/1pgi0sm/2025_day_07_part_2_python_efficient_algorithm_on/
    let mut rays = vec![Ray {row: 0, col: start, weight: 1}];

    // Loop through the manifold line by line
    for _i in 0..manifold.len() - 1  {
        let mut new_rays: Vec<Ray> = Vec::new();
        // Loop through all rays
        for i in 0..rays.len() {
            // If there is a splitter below the ray, split into two new rays with the same weight on either side of the splitter
            if manifold[rays[i].row + 1][rays[i].col] == '^' {
                let new_ray = Ray {row: rays[i].row + 1, col: rays[i].col + 1, weight: rays[i].weight};
                new_rays.push(new_ray);
                rays[i].row += 1;
                rays[i].col -= 1;
                new_rays.push(rays[i]);
            }
            // Else, move the ray down by one row
            else {
                new_rays.push(Ray {row: rays[i].row + 1, col: rays[i].col, weight: rays[i].weight});
            }
        }

        // Check for duplicate rays in new_rays and combine their weights
        let mut ray_map: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
        for ray in new_rays {
            let key = (ray.row, ray.col);
            let entry = ray_map.entry(key).or_insert(0);
            *entry += ray.weight;
        }

        // Return all entries from ray_map back to rays to prepare for the next loop
        rays = ray_map.iter()
            .map(|(&(row, col), &weight)| Ray {row, col, weight})
            .collect();

    }

    // Sum the weights of all rays that reach the bottom row is the total number of possible worlds
    let total: u64 = rays.iter()
        .filter(|ray| ray.row == manifold.len() - 1)
        .map(|ray| ray.weight as u64)
        .sum();
    
    total
}
