use std::collections::{BTreeMap};
use std::fs;
use disjoint::DisjointSetVec;

#[derive(Clone)]
struct Point {
    x: u64,
    y: u64,
    z: u64
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

        points.push(Point { x: coords[0], y: coords[1], z: coords[2] });
    }

    // Calculate the distances between each pair of points
    // The BTreeMap will automatically and efficiently sort the distances so we can easily find the closest pairs later
    let mut distances : BTreeMap<u64, (usize, usize)> = BTreeMap::new();
    for (i, point1) in points.iter().enumerate() {
        for (j, point2) in points.iter().enumerate() {
            // If the points are the same, ignore them
            if i == j {
                continue;
            }
            // Otherwise, calculate the distance and add it to the map
            distances.entry(sq_distance(point1, point2)).or_insert((i, j));
        }
    }

    // Create sets by merging the closest points until they are all in one set
    // The DisjointSetVec allows us to efficiently merge sets and find which set a point belongs to
    let mut circuts: DisjointSetVec<Point> = DisjointSetVec::from(points.clone()); // We have to preserve points so we can go back later and find the x value of the last points merged
    let mut num_sets = points.len();
    for (_, (p1, p2)) in distances.iter() {
        if circuts.join(*p1, *p2) {
            num_sets -= 1;
        }

        // If the number of sets is 1, stop merging
        if num_sets == 1 {
            // Multiply the x coordinates of the last two sets joined
            let x1 = points[*p1].x;
            let x2 = points[*p2].x;

            println!("{}", x1 * x2);

            break;
        }
    }
}

// Calculates the Euclidean distance between two 3d points
// We only care about which points are closer, so we can skip the square root and just return the squared distance
fn sq_distance(a: &Point, b: &Point) -> u64 {
    a.x.abs_diff(b.x).pow(2) +
    a.y.abs_diff(b.y).pow(2) +
    a.z.abs_diff(b.z).pow(2)
}
