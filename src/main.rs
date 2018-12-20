#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

fn main() {
    let input = include_str!("input/day_20.txt");
    println!("start parsing");

    let mut visited = HashMap::new();
    visited.insert((0, 0), [false; 4]);

    let mut positions = vec![(0, 0)];

    let mut backup = vec![];
    backup.push(positions.clone());

    let mut alt: Vec<Vec<(i32, i32)>> = vec![];
    let mut alt_backup = vec![];

    for c in input.chars().skip(1) {
        let dir;
        match c {
            '(' => {
                backup.push(positions.clone());
                alt = vec![];
                continue;
            }
            ')' => {
                for a in alt.into_iter() {
                    positions.reserve(a.len());
                    for pos in a {
                        positions.push(pos);
                    }
                }
                positions.sort_by_key(|p| p.0 * 1000 + p.1);
                positions.dedup();
                alt = alt_backup.pop().unwrap_or(vec![]);
                backup.pop();
                continue;
            }
            '|' => {
                alt.push(positions);
                positions = backup.last().unwrap().clone();
                continue;
            }
            'N' => dir = 0,
            'S' => dir = 2,
            'W' => dir = 3,
            'E' => dir = 1,
            '$' => break,
            c => panic!("unexpected {}", c),
        }
        let delta = [(0i32, -1i32), (1, 0), (0, 1), (-1, 0)][dir];
        for p in &mut positions {
            visited.entry(*p).or_insert([false; 4])[dir] = true;
            p.0 += delta.0;
            p.1 += delta.1;
            visited.entry(*p).or_insert([false; 4])[(dir + 2) % 4] = true;
        }
    }

    println!("done parsing");

    let targets = visited.keys().cloned().collect::<Vec<_>>();
    let paths = dijkstra_search(
        |k| {
            visited[&k]
                .iter()
                .enumerate()
                .filter(|&(_, b)| *b)
                .map(move |(i, _)| match i {
                    0 => (k.0, k.1 - 1),
                    1 => (k.0 + 1, k.1),
                    2 => (k.0, k.1 + 1),
                    3 => (k.0 - 1, k.1),
                    _ => panic!("meh"),
                })
        },
        |_, _| 1,
        |_| true,
        (0, 0),
        &targets,
    );

    let max: usize = paths.values().map(|p| p.cost).max().unwrap();

    println!("max: {}", max);
}
