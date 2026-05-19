use std::cmp::PartialEq;
use std::fs;

struct Point {
    x: u64,
    y: u64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {

    // Read input file
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();

    // Parse each line to a point
    let mut points: Vec<Point> = Vec::new();
    for line in lines {
        let coords: Vec<u64> = line
            .split(',')
            .map(|num| num.trim().parse().expect("Invalid number"))
            .collect();

        points.push(Point { x: coords[0], y: coords[1] });
    }

    // Loop through each pair of points and find the area of the rectangle formed by them
    let mut max_area: u64 = 0;
    for point1 in points.iter() {
        for point2 in points.iter() {
            if point1 == point2 {
                continue;
            }

            max_area = max_area.max(calc_rec(&point1, &point2));
        }
    }

    println!("Maximum area: {}", max_area);
}

// Calculate the area of the rectangle formed by two points
fn calc_rec(p1: &Point, p2: &Point) -> u64 {
    (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1)
}
