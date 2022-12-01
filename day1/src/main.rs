use std::fs;

fn split_numbers(s: &str) -> Vec<i32> {
    s.trim()
        .split("\n")
        .map(|s| s.parse::<i32>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect()
}
fn day1_a(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let blocks: Vec<i32> = contents
        .split("\n\n")
        .map(|s| split_numbers(s))
        .map(|s| s.iter().sum::<i32>())
        .collect();

    println!("max value: {:#?}", blocks.iter().max().unwrap());
}

fn day1_b(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut blocks: Vec<i32> = contents
        .split("\n\n")
        .map(|s| split_numbers(s))
        .map(|s| s.iter().sum::<i32>())
        .collect();

    blocks.sort_by(|a, b| b.cmp(a));

    if blocks.len() < 3 {
        panic!("Less than 3 items in list");
    }

    let result: i32 = blocks.iter().take(3).sum();
    println!("max value of top 3: {:#?}", result);
}

fn main() {
    let filename = "data.txt";
    day1_a(filename);
    day1_b(filename);
}
