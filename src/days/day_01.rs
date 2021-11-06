use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let mut frequency = 0;
    let mut history = HashSet::new();
    for x in input.lines().map(parse).cycle() {
        frequency += x;
        if !history.insert(frequency) {
            pv!(frequency);
            break;
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input.lines().map(parse).sum::<isize>();
    pv!(parsed);
}
