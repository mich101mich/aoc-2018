#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

fn opposite(c: char) -> char {
    if c == 'G' {
        'E'
    } else {
        'G'
    }
}

fn main() {
    let input = include_str!("input/day_15.txt");

    let mut goblins = vec![];
    let mut elves = vec![];

    'all: for attack in 4..50 {
        let mut grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = grid[0].len();
        let height = grid.len();

        goblins = vec![];
        elves = vec![];
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == 'G' {
                    goblins.push((x, y, 200));
                } else if grid[y][x] == 'E' {
                    elves.push((x, y, 200));
                }
            }
        }

        'game: for round in 0..1000 {
            /*for y in 0..height {
                for x in 0..width {
                    print!("{}", grid[y][x]);
                }
                println!();
            }*/
            //println!("{:?}", elves);
            //println!("{:?}", goblins);
            let mut old_grid = grid.clone();
            for y in 0..height {
                for x in 0..width {
                    if (old_grid[y][x] != 'E' && old_grid[y][x] != 'G')
                        || grid[y][x] != old_grid[y][x]
                    {
                        continue;
                    }
                    let me = grid[y][x];
                    let mut pos = (x, y);
                    let mut targets = neighbours((x, y), width, height)
                        .filter(|p| grid[p.1][p.0] == opposite(me))
                        .map(|p| {
                            *if me == 'E' { &goblins } else { &elves }
                                .iter()
                                .find(|o| o.0 == p.0 && o.1 == p.1)
                                .expect("nice")
                        })
                        .collect::<Vec<_>>();

                    if targets.is_empty() {
                        if goblins.is_empty() {
                            println!("{}", round);
                            break 'all;
                        }

                        let destinations = dijkstra_search(
                            |p| neighbours(p, width, height),
                            |_, _| 1,
                            |p| grid[p.1][p.0] == '.',
                            (x, y),
                            &if me == 'E' { &goblins } else { &elves }
                                .iter()
                                .map(|&(x, y, _)| (x, y))
                                .collect::<Vec<_>>(),
                        );
                        if destinations.is_empty() {
                            continue;
                        }
                        let mut destinations = destinations.into_iter().collect::<Vec<_>>();
                        destinations.sort_by_key(|(_, path)| path.cost);
                        let min_dist = destinations[0].1.cost;
                        destinations.retain(|(_, path)| path.cost == min_dist);
                        destinations.sort_by_key(|(_, path)| path[1].0 + path[1].1 * width);
                        let next = destinations[0].1.path[1];
                        grid[next.1][next.0] = grid[y][x];
                        grid[y][x] = '.';
                        if me == 'E' {
                            let hp = elves.iter().find(|p| p.0 == x && p.1 == y).unwrap().2;
                            elves.retain(|p| *p != (x, y, hp));
                            elves.push((next.0, next.1, hp));
                        } else {
                            let hp = goblins.iter().find(|p| p.0 == x && p.1 == y).unwrap().2;
                            goblins.retain(|p| *p != (x, y, hp));
                            goblins.push((next.0, next.1, hp));
                        }
                        pos = next;
                    }

                    targets = neighbours(pos, width, height)
                        .filter(|p| grid[p.1][p.0] == opposite(me))
                        .map(|p| {
                            *if me == 'E' { &goblins } else { &elves }
                                .iter()
                                .find(|o| o.0 == p.0 && o.1 == p.1)
                                .expect("nice")
                        })
                        .collect::<Vec<_>>();

                    if targets.is_empty() {
                        continue;
                    }
                    targets.sort_by_key(|p| p.2);
                    let target = targets[0];
                    let power = if me == 'E' { attack } else { 3 };
                    if target.2 < power {
                        grid[target.1][target.0] = '.';
                        old_grid[target.1][target.0] = '.';
                        if me == 'G' {
                            break 'game;
                        }
                        goblins.retain(|p| *p != target);
                    } else {
                        let other = if me == 'E' { &mut goblins } else { &mut elves };
                        let mut found = false;
                        for o in other {
                            if *o == target {
                                o.2 -= power;
                                //println!("{:?}", *o);
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            panic!();
                        }
                    }
                }
            }
        }
    }
    if elves.is_empty() {
        // 240030, 239760, 237363, 237096, 236829
        let mut result = 0;
        for g in goblins {
            result += g.2;
        }
        println!("{}", result);
        return;
    }
    if goblins.is_empty() {
        let mut result = 0;
        for e in elves {
            result += e.2;
        }
        println!("{}", result);
        return;
    }
}

fn neighbours(
    (x, y): (usize, usize),
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut n = vec![];
    if y > 0 {
        n.push((x, y - 1));
    }
    if x > 0 {
        n.push((x - 1, y));
    }
    if x < width - 1 {
        n.push((x + 1, y));
    }
    if y < height - 1 {
        n.push((x, y + 1));
    }
    n.into_iter()
}

#[allow(unused)]
fn manhatten(p1: (usize, usize), p2: (usize, usize)) -> usize {
    ((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as usize
}

#[allow(unused)]
fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
    std::iter::repeat(std::iter::repeat(value).take(height).collect::<Vec<T>>())
        .take(width)
        .collect()
}

#[allow(unused)]
#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
