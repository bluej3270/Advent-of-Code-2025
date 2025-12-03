use std::fs;

fn main() {
    let mut total: u64 = 0;

    let input = fs::read_to_string("input.txt").expect("File not found");
    let ranges = input.split(',').collect::<Vec<&str>>();

    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let start :u64 = start.parse::<u64>().unwrap();
        let end :u64 = end.parse::<u64>().unwrap();

        for value in start..end {
            total += if check_product(value) {value} else {0}
        }
    }

    println!("total: {}", total);

}

// Check it an id is invalid (returns true) or not (returns false) (Part 1)
// fn check_product(product: u64) -> bool {
//     let string_product = product.to_string();
//     if string_product.len() % 2 == 0 {
//         let (first, rest) = string_product.split_at(string_product.len() / 2);
//         first.parse::<u64>().unwrap() == rest.parse::<u64>().unwrap() // If first and rest are the same, this id is made only of a sequence of digits repeated twice.
//     } else {
//         false // Odd length ids are always valid
//     }
//
// }

// Check if an id is invalid (returns true) or not (returns false) (Part 2)
fn check_product(product: u64) -> bool {
    let string_product = product.to_string();

    for l in 1..(string_product.len()/2)+1 {
        let check_string = &string_product[0..l].to_string().repeat(string_product.len()/l);
        // println!("String Product: {}, Check String: {}", string_product, check_string);
        if string_product == *check_string {

            return true
        }
    }

    false
}