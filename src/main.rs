use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let mut f = 0;
    for n in input.lines() {
        if n.starts_with("+") {
            f += i32::from_str(&n[1..]).unwrap();
        } else {
            f -= i32::from_str(&n[1..]).unwrap();
        }
    }
    println!("{}", f);
}
