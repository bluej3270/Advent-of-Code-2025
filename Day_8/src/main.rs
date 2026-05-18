use std::collections::{BTreeMap, HashMap};
use std::fs;
use disjoint::DisjointSetVec;

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

    // Create sets by merging the closest points (1000 times)
    // The DisjointSetVec allows us to efficiently merge sets and find which set a point belongs to
    let mut circuts: DisjointSetVec<Point> = DisjointSetVec::from(points);
    for (_, (i, j)) in distances.iter().take(1000) {
        circuts.join(*i, *j);
    }

    // Calculate the sizes of every set / circuit
    // All entries in one set will have the same set root, so we will use that to identify each set
    let mut sizes: HashMap<usize, u64> = HashMap::new();
    for i in 0..circuts.len() {
        *sizes.entry(circuts.root_of(i)).or_insert(0) += 1;
    }

    // Multiply the sizes of the 3 largest sets together
    let mut size_values: Vec<u64> = sizes.values().copied().collect();
    size_values.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    let product: u64 = size_values.iter().take(3).product();
    println!("Product: {}", product);
}

// Calculates the Euclidean distance between two 3d points
// We only care about which points are closer, so we can skip the square root and just return the squared distance
fn sq_distance(a: &Point, b: &Point) -> u64 {
    a.x.abs_diff(b.x).pow(2) +
    a.y.abs_diff(b.y).pow(2) +
    a.z.abs_diff(b.z).pow(2)
}
