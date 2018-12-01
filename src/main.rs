use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input/day_1.txt");
    let mut f = 0;
    let mut history = HashSet::new();
    loop {
        for n in input.lines() {
            if n.starts_with("+") {
                f += i32::from_str(&n[1..]).unwrap();
            } else {
                f -= i32::from_str(&n[1..]).unwrap();
            }
            if !history.insert(f) {
                println!("{}", f);
                return;
            }
        }
    }
}
