#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_8.txt");

    let numbers = input
        .split(' ')
        .map(|s| usize::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let mut sum = 0;

    find_node_end(&numbers, &mut sum);
    println!("{}", sum);
}

fn find_node_end(numbers: &[usize], sum: &mut usize) -> usize {
    let nodes = numbers[0];
    let meta = numbers[1];
    let mut cur_index = 2;
    for _ in 0..nodes {
        let end = find_node_end(&numbers[cur_index..], sum);
        cur_index += end;
    }
    for i in 0..meta {
        *sum += numbers[cur_index + i];
    }
    cur_index + meta
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
