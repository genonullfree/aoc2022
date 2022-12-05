use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq)]
enum WLD {
    Lose,
    Draw,
    Win,
}

fn main() {
    println!("Hello, AoC2022!");

    let mut total_score = 0usize;

    let file = File::open("day02.txt").unwrap();
    let input = BufReader::new(file).lines();

    for line in input {
        if let Ok(mut data) = line {
            if data.is_empty() {
                panic!();
            }
            let us = wld_pick(data.pop().unwrap());
            let _ = data.pop();
            let them = them_pick(data.pop().unwrap());

            total_score += calc(them, us);
        }
    }

    println!("{}", total_score);
}

fn calc(them: RPS, you: WLD) -> usize {
    let mut us_picked = match them {
        RPS::Rock => match you {
            WLD::Lose => RPS::Scissors,
            WLD::Draw => RPS::Rock,
            WLD::Win => RPS::Paper,
        },
        RPS::Paper => match you {
            WLD::Lose => RPS::Rock,
            WLD::Draw => RPS::Paper,
            WLD::Win => RPS::Scissors,
        },
        RPS::Scissors => match you {
            WLD::Lose => RPS::Paper,
            WLD::Draw => RPS::Scissors,
            WLD::Win => RPS::Rock,
        },
    };

    let mut score = match you {
        WLD::Lose => 0,
        WLD::Draw => 3,
        WLD::Win => 6,
    };

    score += match us_picked {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    score
}

fn them_pick(choice: char) -> RPS {
    match choice {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!(),
    }
}

fn wld_pick(choice: char) -> WLD {
    match choice {
        'X' => WLD::Lose,
        'Y' => WLD::Draw,
        'Z' => WLD::Win,
        _ => panic!(),
    }
}
