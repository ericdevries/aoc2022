use std::{collections::HashSet, fs};

#[derive(Debug)]
struct RPS {
    a: char,
    b: char,
}

fn char_to_num(c: char) -> i32 {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => 0,
    }
}

fn result_to_char(c: char, result: char) -> char {
    match (c, result) {
        // X = loss
        ('A', 'X') => 'C',
        ('B', 'X') => 'A',
        ('C', 'X') => 'B',
        // Z = win
        ('A', 'Z') => 'B',
        ('B', 'Z') => 'C',
        ('C', 'Z') => 'A',
        // Y = draw, so return the same one as the other player played
        _ => c,
    }
}

fn points(rps: &RPS) -> i32 {
    if rps.a as i32 == rps.b as i32 - 23 {
        return 3;
    }

    match rps {
        RPS { a: 'A', b: 'Y' } => 6,
        RPS { a: 'B', b: 'Z' } => 6,
        RPS { a: 'C', b: 'X' } => 6,

        _ => 0,
    }
}

fn day1_b(lines: &Vec<String>) {
    let mut groups: Vec<Vec<String>> = Vec::new();
    for i in 0..(lines.len() / 3) {
        let x = lines[i * 3..(i + 1) * 3].to_vec();

        groups.push(x);
    }

    let result: i32 = groups
        .iter()
        .map(|s| {
            let sets: Vec<HashSet<char>> = s
                .iter()
                .map(|x| x.chars().into_iter().collect::<HashSet<char>>())
                .collect();

            if let Some((first, rest)) = sets.split_first() {
                let second = rest.get(0).unwrap();
                let third = rest.get(1).unwrap();

                let inter = first
                    .intersection(&second)
                    .copied()
                    .collect::<HashSet<char>>();

                let final_result = inter
                    .intersection(&third)
                    .copied()
                    .collect::<HashSet<char>>();

                let c = final_result.iter().nth(0).unwrap();
                return letter_to_point(c);
            } else {
                return 0;
            }
        })
        .sum();

    println!("result: {:?}", result);
}

fn day1_a(lines: &Vec<String>) {
    println!("line: {:?}", lines);
    let result: Vec<i32> = lines
        .iter()
        .map(|s| {
            let mid = s.len() / 2;
            (s.get(0..mid).unwrap(), s.get(mid..).unwrap())
        })
        .map(|(s1, s2)| {
            println!("comparing lines {} and {}", s1, s2);
            let mut res = same_letters(s1, s2);
            res.dedup();
            return res;
        })
        .map(|s| {
            let res: Vec<i32> = s.iter().map(letter_to_point).collect();
            return res;
        })
        .map(|s| return s.iter().sum::<i32>())
        .collect();

    let sum: i32 = result.iter().sum();
    println!("answer: {}", sum);
}

fn letter_to_point(c: &char) -> i32 {
    let i = *c as i32;

    if i >= 65 && i <= 90 {
        return i - 38;
    } else {
        return i - 96;
    }
}
fn same_letters(s1: &str, s2: &str) -> Vec<char> {
    let s2chars: Vec<char> = s2.chars().into_iter().collect();
    return s1.chars().filter(|s| s2chars.contains(s)).collect();
}

fn get_lines(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    return contents
        .split("\n")
        .filter(|s| s.trim().len() > 0)
        .map(|s| String::from(s))
        .collect();
}

fn main() {
    let data = get_lines("data.txt");
    day1_a(&data);
    day1_b(&data);
}
