#![allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

enum Dir {
    Up,
    Right,
    Down,
    Left,
}
use crate::Dir::*;

fn main() {
    let input = include_str!("input/day_13.txt");

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut carts = vec![];

    let h = grid.len();
    let w = grid[0].len();

    for y in 0..h {
        for x in 0..w {
            match grid[y][x] {
                '>' => {
                    carts.push((x, y, Right, 0));
                    grid[y][x] = '-';
                }
                '<' => {
                    carts.push((x, y, Left, 0));
                    grid[y][x] = '-';
                }
                '^' => {
                    carts.push((x, y, Up, 0));
                    grid[y][x] = '|';
                }
                'v' => {
                    carts.push((x, y, Down, 0));
                    grid[y][x] = '|';
                }
                _ => {}
            }
        }
    }

    let mut count = 0;
    loop {
        count += 1;

        carts.sort_by_key(|cart| cart.0 + cart.1 * h);

        for i in 0..carts.len() {
            {
                let cart = &mut carts[i];
                match cart.2 {
                    Right => {
                        cart.0 += 1;
                        cart.2 = next_dir(grid[cart.1][cart.0], &Right, &mut cart.3);
                    }
                    Left => {
                        cart.0 -= 1;
                        cart.2 = next_dir(grid[cart.1][cart.0], &Left, &mut cart.3);
                    }
                    Up => {
                        cart.1 -= 1;
                        cart.2 = next_dir(grid[cart.1][cart.0], &Up, &mut cart.3);
                    }
                    Down => {
                        cart.1 += 1;
                        cart.2 = next_dir(grid[cart.1][cart.0], &Down, &mut cart.3);
                    }
                }
            }
            for j in 0..carts.len() {
                if i != j && carts[i].0 == carts[j].0 && carts[i].1 == carts[j].1 {
                    println!("{},{}", carts[i].0, carts[i].1);
                    return;
                }
            }
        }
    }
}

fn next_dir(c: char, current: &Dir, intersect: &mut usize) -> Dir {
    match current {
        Right => match c {
            '/' => Up,
            '\\' => Down,
            '+' => {
                let d;
                match intersect {
                    0 => d = Up,
                    1 => d = Right,
                    _ => d = Down,
                }
                *intersect = (*intersect + 1) % 3;
                d
            }
            '-' => Right,
            _ => panic!("invalid char on track"),
        },
        Left => match c {
            '/' => Down,
            '\\' => Up,
            '+' => {
                let d;
                match intersect {
                    0 => d = Down,
                    1 => d = Left,
                    _ => d = Up,
                }
                *intersect = (*intersect + 1) % 3;
                d
            }
            '-' => Left,
            _ => panic!("invalid char on track"),
        },
        Up => match c {
            '/' => Right,
            '\\' => Left,
            '+' => {
                let d;
                match intersect {
                    0 => d = Left,
                    1 => d = Up,
                    _ => d = Right,
                }
                *intersect = (*intersect + 1) % 3;
                d
            }
            '|' => Up,
            _ => panic!("invalid char on track"),
        },
        Down => match c {
            '/' => Left,
            '\\' => Right,
            '+' => {
                let d;
                match intersect {
                    0 => d = Right,
                    1 => d = Down,
                    _ => d = Left,
                }
                *intersect = (*intersect + 1) % 3;
                d
            }
            '|' => Down,
            _ => panic!("invalid char on track"),
        },
    }
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
