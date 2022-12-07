const INPUT: &str = include_str!("../../day06.txt");

fn main() {
    let mut window = Vec::<char>::new();
    window.resize(14, '0');
    for (n, i) in INPUT.chars().enumerate() {
        if n < 14 {
            shift_in(&mut window, i);
        }
        if are_unique(&window) {
            println!("marker at: {}", n);
            break;
        }
        shift_in(&mut window, i);
    }
}

fn shift_in(window: &mut Vec<char>, input: char) {
    window.remove(0);
    window.push(input);
}

fn are_unique(window: &Vec<char>) -> bool {
    for i in window {
        let mut count = 0;
        for c in window {
            if i == c {
                count += 1;
            }
        }
        if count != 1 {
            return false;
        }
    }

    true
}
