use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Elf {
    calories: usize,
}

fn main() {
    println!("Hello, AoC2022!");

    let mut elf = Vec::<Elf>::new();
    let mut current_elf = Elf::default();

    let file = File::open("day01.txt").unwrap();
    let input = BufReader::new(file).lines();

    for data in input.flatten() {
        if data.is_empty() {
            elf.push(current_elf.clone());
            current_elf = Elf::default();
            continue;
        }
        current_elf.calories += data.parse::<usize>().unwrap();
    }

    elf.sort_by_key(|e| e.calories);

    let mut total = 0;
    for _ in 0..3 {
        let e = elf.pop().unwrap();
        total += e.calories;
    }
    println!("{}", total);
}
