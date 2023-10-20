use crate::utils::*;

fn calc_score(state: &[bool], num_iter: usize, padding: usize) -> isize {
    state
        .iter()
        .enumerate()
        .map(|(i, b)| (i as isize - (num_iter * padding) as isize, b))
        .filter(|(_, b)| **b)
        .map(|(i, _)| i)
        .sum::<isize>()
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut iter = input.lines();
    let line = iter.next().unwrap();
    let mut state = sscanf!(line, "initial state: {HashtagLine}").unwrap().0;
    iter.next().unwrap();

    let rules = iter
        .map(|l| sscanf!(l, "{HashtagLine} => {char}").unwrap())
        .map(|(l, r)| (l.0, r == '#'))
        .to_map();

    let num_iter = 500;
    let padding = 3;

    state = std::iter::repeat(false)
        .take(num_iter * padding)
        .chain(state)
        .chain(std::iter::repeat(false).take(num_iter * padding))
        .collect();

    let mut next_gen = state.clone();
    for gen in 0..num_iter {
        for (i, w) in state.windows(5).enumerate() {
            next_gen[i + 2] = rules[w];
        }

        if next_gen
            .iter()
            .skip(1)
            .zip(state.iter())
            .all(|(a, b)| a == b)
        {
            let start = calc_score(&state, num_iter, padding) as u128;
            let next = calc_score(&next_gen, num_iter, padding) as u128;
            let step = next - start;
            let duration = 50_000_000_000 - gen as u128;
            let end = start + duration * step;
            pv!(end);
            break;
        }

        std::mem::swap(&mut state, &mut next_gen);
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut iter = input.lines();
    let line = iter.next().unwrap();
    let mut state = sscanf!(line, "initial state: {HashtagLine}").unwrap().0;
    iter.next().unwrap();

    let rules = iter
        .map(|l| sscanf!(l, "{HashtagLine} => {char}").unwrap())
        .map(|(l, r)| (l.0, r == '#'))
        .to_map();

    let num_iter = 20;
    let padding = 2;

    state = std::iter::repeat(false)
        .take(num_iter * padding)
        .chain(state)
        .chain(std::iter::repeat(false).take(num_iter * padding))
        .collect();

    let mut next_gen = state.clone();
    for _ in 0..num_iter {
        for (i, w) in state.windows(5).enumerate() {
            next_gen[i + 2] = rules[w];
        }
        std::mem::swap(&mut state, &mut next_gen);
    }
    let sum = calc_score(&state, num_iter, padding);
    pv!(sum);
}
