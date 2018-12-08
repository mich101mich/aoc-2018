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

    let mut index = 0;

    let value = get_node_value(&numbers, &mut index);
    println!("{}", value);
}

fn get_node_value(numbers: &[usize], index: &mut usize) -> usize {
    let nodes = numbers[*index];
    let meta = numbers[*index + 1];
    *index += 2;

    if nodes == 0 {
        let mut sum = 0;
        for i in 0..meta {
            sum += numbers[*index + i];
        }
        *index += meta;
        return sum;
    }

    let mut values = vec![];
    for _ in 0..nodes {
        let value = get_node_value(numbers, index);
        values.push(value);
    }
    let mut sum = 0;
    for i in 0..meta {
        let data = numbers[*index + i] - 1;
        if let Some(v) = values.get(data) {
            sum += v;
        }
    }
    *index += meta;
    sum
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
