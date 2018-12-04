#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input/day_4.txt");

    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort_by_key(|line| {
        let year = usize::from_str(&line[1..5]).unwrap();
        let month = usize::from_str(&line[6..8]).unwrap();
        let day = usize::from_str(&line[9..11]).unwrap();
        let hour = usize::from_str(&line[12..14]).unwrap();
        let minute = usize::from_str(&line[15..17]).unwrap();
        (((year * 100 + month) * 100 + day) * 100 + hour) * 100 + minute
    });

    let mut guards: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut cur_guard = 0;
    let mut fell_asleep = 0;
    for line in lines.iter() {
        if let Some(e) = line.find(" begins") {
            cur_guard = usize::from_str(&line[26..e]).unwrap();
            continue;
        }
        if let Some(_) = line.find("falls") {
            fell_asleep = usize::from_str(&line[15..17]).unwrap();
            continue;
        }
        if let Some(_) = line.find("wakes") {
            let wakes = usize::from_str(&line[15..17]).unwrap();
            let list = guards
                .entry(cur_guard)
                .or_insert_with(|| std::iter::repeat(0).take(60).collect());
            for i in fell_asleep..wakes {
                list[i] += 1;
            }
            continue;
        }
    }
    let mut max_i = 0;
    let mut max = 0;
    for (guard, list) in guards.iter() {
        let count = list.iter().sum();
        if count > max {
            max_i = *guard;
            max = count;
        }
    }
    let guard_id = max_i;
    let list = &guards[&max_i];
    max = 0;
    max_i = 0;
    for (i, v) in list.iter().enumerate() {
        if *v > max {
            max = *v;
            max_i = i;
        }
    }
    println!("{:?}", guard_id);
    println!("{:?}", max_i);
    println!("{:?}", max_i * guard_id);
}

#[allow(unused)]
fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
    std::iter::repeat(std::iter::repeat(value).take(width).collect::<Vec<T>>())
        .take(height)
        .collect()
}
