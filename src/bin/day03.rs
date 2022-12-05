const INPUT: &str = include_str!("../../day03.txt");
const SCORE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let lines: Vec<&str> = INPUT.trim().lines().collect();
    let mut score = 0;
    for l in lines.chunks(3) {
        score += analyze_lines(l);
    }

    println!("score: {}", score);
}

fn analyze_lines(input: &[&str]) -> usize {
    if input.len() != 3 {
        panic!();
    }

    let a = input[0];
    let b = input[1];
    let c = input[2];

    for i in a.chars() {
        if b.contains(i) && c.contains(i) {
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
