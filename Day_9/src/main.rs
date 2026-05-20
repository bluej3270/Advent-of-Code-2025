use std::cmp::{max, min, PartialEq};
use std::fs;

#[derive(PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

struct Line {
    start: Point,
    end: Point,
}

struct Rectangle {
    top_left: Point,
    top_right: Point,
    bottom_left: Point,
    bottom_right: Point,
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

    // Also form a list of all the lines formed by consecutive points, representing the bounding shape
    let shape: Vec<Line> = points.iter().zip(points.iter().cycle().skip(1))
        .map(|(start, end)| Line { start: Point { x: start.x, y: start.y }, end: Point { x: end.x, y: end.y } })
        .collect();

    // Loop through each pair of points and find the area of the rectangle formed by them
    let mut max_area: u64 = 0;
    for point1 in points.iter() {
        for point2 in points.iter() {
            // If both points are the same point, skip them
            if point1 == point2 {
                continue;
            }

            let area = calc_rec(&point1, &point2);

            // If the area is bigger then the current max_area, check to see if this rectangle is legal (bound by the outer polygon). If it is, update max_area
            if area > max_area {
                let rectangle: Rectangle = construct_rec(&point1, &point2);
                let mut legal: bool = true;
                // Loop through all the lines in the bounding shape
                for line in shape.iter() {
                    // If line.start is inside the rectangle, the rectangle is not legal.
                    if check_inside(&line.start, &rectangle) {
                        legal = false;
                        break;
                    }

                    // If the line crosses through the rectangle, the rectangle is not legal
                    if check_cross(line, &rectangle) {
                        legal = false;
                        break;
                    }
                }
                if legal {
                    max_area = area;
                }
            }
        }
    }

    println!("Maximum area: {}", max_area);
}

// Calculate the area of the rectangle formed by two points
fn calc_rec(p1: &Point, p2: &Point) -> u64 {
    (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1)
}

// Checks if a line intersects with a rectangle
fn check_cross(line: &Line, rect: &Rectangle) -> bool {
    let left = rect.top_left.x;
    let right = rect.top_right.x;
    let top = rect.top_left.y;
    let bottom = rect.bottom_left.y;

    // Line is Horizontal
    if line.start.y == line.end.y {
        let y = line.start.y;

        // Segment is outside vertical range of rectangle
        if y <= top || y >= bottom {
            return false;
        }

        // Check if the line intersects with rectangle horizontally
        let min_x = line.start.x.min(line.end.x);
        let max_x = line.start.x.max(line.end.x);

        return max_x > left && min_x < right;
    }

    // Line is Vertical
    if line.start.x == line.end.x {
        let x = line.start.x;

        // Segment is outside horizontal range of rectangle
        if x <= left || x >= right {
            return false;
        }

        // Check if the line intersects with rectangle vertically
        let min_y = line.start.y.min(line.end.y);
        let max_y = line.start.y.max(line.end.y);

        return max_y > top && min_y < bottom;
    }

    false
}

// Checks if a point is inside a rectangle (for this problem, on the border is not inside, so should return false)
fn check_inside(p: &Point, rectangle: &Rectangle) -> bool {
    p.x > rectangle.top_left.x
        && p.x < rectangle.top_right.x
        && p.y > rectangle.top_left.y
        && p.y < rectangle.bottom_left.y
}

// Constructs a rectangle from two points
fn construct_rec(p1: &Point, p2: &Point) -> Rectangle {
    let top_left = Point { x: min(p1.x, p2.x), y: min(p1.y, p2.y) }; // Top Left = smallest x, smallest y
    let top_right = Point { x: max(p1.x, p2.x), y: min(p1.y, p2.y) }; // Top Right = largest x, smallest y
    let bottom_left = Point { x: min(p1.x, p2.x), y: max(p1.y, p2.y) }; // Bottom Left = smallest x, largest y
    let bottom_right = Point { x: max(p1.x, p2.x), y: max(p1.y, p2.y) }; // Bottom Right = largest x, largest y
    Rectangle { top_left, top_right, bottom_left, bottom_right }

}