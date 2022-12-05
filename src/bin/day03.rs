const INPUT: &str = include_str!("../../day03.txt");
const SCORE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let lines: Vec<&str> = INPUT.trim().lines().collect();
    let mut score = 0;
    for l in lines {
        score += analyze_line(l);
    }

    println!("score: {}", score);
}

fn analyze_line(input: &str) -> usize {
    let length = input.len();
    if length % 2 != 0 {
        panic!();
    }

    let a = &input[..length / 2];
    let b = &input[length / 2..];

    find_dupe(a, b)
}

fn find_dupe(a: &str, b: &str) -> usize {
    for i in a.chars() {
        if b.contains(i) {
            return get_score(i);
        }
    }
    panic!();
}

fn get_score(a: char) -> usize {
    for (n, i) in SCORE.chars().enumerate() {
        if i == a {
            return n + 1;
        }
    }
    panic!();
}
