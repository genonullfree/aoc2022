const INPUT: &str = include_str!("../../day04.txt");

#[derive(Default, Debug, PartialEq, Eq)]
struct Range {
    data: Vec<usize>,
}

fn main() {
    let lines: Vec<&str> = INPUT.trim().lines().collect();
    let mut score = 0;
    for l in lines {
        if analyze_line(l) {
            score += 1;
        }
    }

    println!("score: {}", score);
}

fn analyze_line(input: &str) -> bool {
    if input.len() <= 0 {
        panic!();
    }

    let line: Vec<&str> = input.split(',').collect();
    let first = get_range(line[0]);
    let second = get_range(line[1]);

    vec_contains(&first, &second) || vec_contains(&second, &first)
}

fn vec_contains(a: &[usize], b: &[usize]) -> bool {
    for (n, i) in a.iter().enumerate() {
        if !b.contains(i) {
            break;
        }
        if n == a.len() - 1 {
            return true;
        }
    }

    false
}

fn get_range(input: &str) -> Vec<usize> {
    let range: Vec<&str> = input.split('-').collect();
    let mut counter = usize::from_str_radix(range[0], 10).unwrap();
    let end = usize::from_str_radix(range[1], 10).unwrap();
    let mut out = Vec::<usize>::new();

    loop {
        out.push(counter);
        if counter == end {
            break;
        }
        counter += 1;
    }

    out
}
