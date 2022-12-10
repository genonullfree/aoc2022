const INPUT: &str = include_str!("../../day08.txt");

#[derive(Debug, Clone)]
struct Tree {
    value: u8,
    visible: bool,
}

impl Tree {
    fn new(value: char) -> Self {
        Self {
            value: (value as u8) - 0x30,
            visible: false,
        }
    }

    fn visibility(&mut self, prev: u8) -> u8 {
        if prev < self.value {
            self.visible = true;
            return self.value;
        }
        prev
    }
}

fn main() {
    let mut graph = Vec::<Vec<Tree>>::new();
    for i in INPUT.lines() {
        let mut row = Vec::<Tree>::new();
        for c in i.chars() {
            row.push(Tree::new(c));
        }
        graph.push(row);
    }

    setup(&mut graph);
    calc_visible(&mut graph);
    let count = count_visible(&graph);

    println!("count: {}", count);
}

fn count_visible(input: &Vec<Vec<Tree>>) -> usize {
    let mut count = 0;
    for i in input {
        count += i
            .iter()
            .fold(0, |acc, x| if x.visible { acc + 1 } else { acc });
    }
    count
}

fn calc_visible(input: &mut Vec<Vec<Tree>>) {
    for i in input.iter_mut() {
        for _ in 0..2 {
            i.reverse();
            let mut height = 0;
            for j in i.iter_mut() {
                height = j.visibility(height);
            }
        }
    }

    for i in 0..input.len() {
        let mut height = 0;
        for j in input.iter_mut() {
            height = j[i].visibility(height);
        }
        height = 0;
        for j in input.iter_mut().rev() {
            height = j[i].visibility(height);
        }
    }
}

fn setup(input: &mut Vec<Vec<Tree>>) {
    for i in input[0].iter_mut() {
        i.visible = true;
    }

    for i in input.last_mut().unwrap() {
        i.visible = true;
    }

    for v in input.iter_mut() {
        v[0].visible = true;
        v.last_mut().unwrap().visible = true;
    }
}
