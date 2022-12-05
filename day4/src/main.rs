use std::{collections::HashSet, fs};

type Overlap = (i32, i32);

#[derive(Debug)]
struct Assignment {
    a: Overlap,
    b: Overlap,
}

impl Assignment {
    fn contains_eachother(&self) -> bool {
        return (self.b.0 >= self.a.0 && self.b.1 <= self.a.1)
            || (self.a.0 >= self.b.0 && self.a.1 <= self.b.1);
    }

    fn overlaps_eachother(&self) -> bool {
        return !(self.a.0 > self.b.1 || self.a.1 < self.b.0);
    }
}

fn day1_a(lines: &Vec<Assignment>) {
    let result = lines.iter().filter(|s| s.contains_eachother()).count();

    println!("result: {:?}", result);
}

fn day1_b(lines: &Vec<Assignment>) {
    let result = lines.iter().filter(|s| s.overlaps_eachother()).count();

    println!("result: {:?}", result);
}

fn parse_assignment(s: &str) -> (i32, i32) {
    let res = s.split_once("-").unwrap();
    return (res.0.parse::<i32>().unwrap(), res.1.parse::<i32>().unwrap());
}

fn get_lines(file: &str) -> Vec<Assignment> {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    return contents
        .split("\n")
        .filter(|s| s.trim().len() > 0)
        .map(|s| {
            return match s.split_once(",") {
                Some((f, s)) => Some(Assignment {
                    a: parse_assignment(f),
                    b: parse_assignment(s),
                }),
                None => None,
            };
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect();
}

fn main() {
    let data = get_lines("data.txt");
    day1_a(&data);
    day1_b(&data);
}
