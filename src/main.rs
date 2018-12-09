#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let (players, last) = include!("input/day_9.txt");
    //let (players, last) = (9, 25);

    let mut scores = std::iter::repeat(0).take(players).collect::<Vec<usize>>();
    let mut marbles = std::collections::VecDeque::new();
    marbles.push_back(0);
    marbles.push_back(2);
    marbles.push_back(1);
    //let mut marbles = vec![0, 2, 1];
    let mut current = 1;
    let mut player = 3;

    for i in 3..=(last * 100) {
        if i % 10000 == 0 {
            println!("at {}", i);
        }
        if i % 23 == 0 {
            scores[player] += i;
            let to_remove = (current + marbles.len() - 7) % marbles.len();
            scores[player] += marbles.remove(to_remove).unwrap();
            current = to_remove % marbles.len();
            player = (player + 1) % players;
            continue;
        }

        let next = (current + 2) % marbles.len();
        if next == 0 {
            marbles.push_back(i);
            current = marbles.len() - 1;
        } else {
            marbles.insert(next, i);
            current = next;
        }

        player = (player + 1) % players;
    }
    println!("{}", scores.iter().max().unwrap());
}

#[allow(unused)]
fn manhatten(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[allow(unused)]
fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
    std::iter::repeat(std::iter::repeat(value).take(height).collect::<Vec<T>>())
        .take(width)
        .collect()
}
