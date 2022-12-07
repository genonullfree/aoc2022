const INPUT: &str = include_str!("../../day05.txt");

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct Stack {
    data: Vec<char>,
}

fn main() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let (setup, moves) = split_input(lines);
    let mut stacks = setup_stacks(setup);
    execute_moves(&mut stacks, moves);
    println!("Final stacks\n{:?}", stacks);

    let mut output: String = "".to_string();
    for mut i in stacks {
        output.push(i.data.pop().unwrap());
    }

    println!("Output: {}", output);
}

fn execute_moves(stacks: &mut Vec<Stack>, input: Vec<&str>) {
    for l in input {
        let command: Vec<&str> = l.split(' ').collect();
        let num = command[1].parse::<usize>().unwrap();
        let from = command[3].parse::<usize>().unwrap() - 1;
        let to = command[5].parse::<usize>().unwrap() - 1;
        execute(stacks, num, from, to);
    }
}

fn execute(stacks: &mut Vec<Stack>, num: usize, from: usize, to: usize) {
    for i in 0..num {
        let d = stacks[from].data.pop().unwrap();
        stacks[to].data.push(d);
    }
}

fn setup_stacks(input: Vec<&str>) -> Vec<Stack> {
    let mut out = Vec::<Stack>::new();
    let size = (input[0].len() + 1) / 4;
    out.resize(size, Stack::default());

    for line in input {
        let mut token = line.char_indices();
        loop {
            let data = token.next();
            if let Some((_, '[')) = data {
                let (n, c) = token.next().unwrap();
                out[(n - 1) / 4].data.push(c);
            } else if data.is_none() {
                break;
            }
        }
    }

    for t in &mut out {
        t.data.reverse();
    }
    out
}

fn split_input(input: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut a = Vec::<&str>::new();
    let mut b = Vec::<&str>::new();
    let mut switch = false;

    for i in input {
        if i.len() == 0 {
            switch = true;
            continue;
        }
        match switch {
            false => a.push(i),
            true => b.push(i),
        };
    }

    (a, b)
}

fn analyze_line(_input: &str) -> bool {
    false
}
