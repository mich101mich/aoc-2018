#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_7.txt");

    let mut letters = HashSet::new();

    let deps = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            (chars.nth(5).unwrap(), chars.nth(30).unwrap())
        })
        .collect::<Vec<_>>();
    let mut tree = HashMap::new();

    for (f, s) in deps {
        letters.insert(f);
        letters.insert(s);
        let list = tree.entry(s).or_insert_with(HashSet::new);
        list.insert(f);
    }

    let mut letters = letters.into_iter().collect::<Vec<_>>();
    letters.sort();
    println!("{:?}", letters);
    'search: while !tree.is_empty() {
        for l in letters.clone().iter() {
            if let Some(list) = tree.get(l) {
                if list.is_empty() {
                    tree.remove(l);
                    print!("{}", l);
                    std::io::stdout().flush();
                    letters.retain(|c| c != l);
                    for list in tree.values_mut() {
                        list.remove(l);
                    }
                    continue 'search;
                }
            } else {
                print!("{}", l);
                std::io::stdout().flush();
                letters.retain(|c| c != l);
                for list in tree.values_mut() {
                    list.remove(l);
                }
                continue 'search;
            }
        }
    }
    println!();
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
