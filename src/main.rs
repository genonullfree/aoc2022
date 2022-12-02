use std::fs::File;
use std::io::{BufReader, BufRead, Read};

#[derive(Debug, Default)]
struct Elf {
    calories: usize,
}

fn main() {
    println!("Hello, AoC2022!");

    let mut elf = Elf::default();
    let mut current_elf = Elf::default();

    let mut file = File::open("list.txt").unwrap();
    let input = BufReader::new(file).lines();

    for line in input {
        if let Ok(data) = line {
            if data.is_empty() {
                if current_elf.calories > elf.calories {
                    elf = current_elf;
                }
                current_elf = Elf::default();
                continue;
            }
            current_elf.calories += data.parse::<usize>().unwrap();
        }

    }

    println!("{:?}", elf);
}
