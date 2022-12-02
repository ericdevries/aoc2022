use std::fs;

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

fn day1_a(file_path: &str) {
    let rounds: i32 = get_lines(file_path)
        .iter()
        .map(|round| {
            // second char dictates hand played
            let mine = char_to_num(round.b);
            let points = points(&round) + mine;
            return points;
        })
        .sum();

    println!("result: {}", rounds);
}

fn day1_b(file_path: &str) {
    let rounds: i32 = get_lines(file_path)
        .iter()
        .map(|round| {
            // the second character dictates the outcome, not the hand played
            let win = match round.b {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => 0,
            };

            // based on outcome get what the protagonist played, and get the score for that (1,2,3)
            let symbol = char_to_num(result_to_char(round.a, round.b));

            return win + symbol;
        })
        .sum();

    println!("result: {}", rounds);
}

fn get_lines(file: &str) -> Vec<RPS> {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    let rounds = contents
        .split("\n")
        .filter(|s| s.trim().len() > 0)
        .map(|s| {
            // probably should do some kind of char to int conversion and subtract 24 (X - A)
            let a = s.chars().nth(0);
            let b = s.chars().nth(2);

            return RPS {
                a: a.unwrap(),
                b: b.unwrap(),
            };
        })
        .collect();

    return rounds;
}
fn main() {
    let filename = "data.txt";
    day1_a(filename);
    day1_b(filename);
}
