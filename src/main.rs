#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]

use crate::Dir::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

mod utils;
use crate::utils::*;

fn count_grid(grid: &[Vec<char>], x: usize, y: usize, target: char) -> usize {
    let w = grid.len();
    let h = grid[0].len();
    let mut count = 0;
    if x > 0 {
        if y > 0 {
            count += (grid[y - 1][x - 1] == target) as usize;
        }
        count += (grid[y][x - 1] == target) as usize;
        if y < h - 1 {
            count += (grid[y + 1][x - 1] == target) as usize;
        }
    }
    if y > 0 {
        count += (grid[y - 1][x] == target) as usize;
    }
    if y < h - 1 {
        count += (grid[y + 1][x] == target) as usize;
    }
    if x < w - 1 {
        if y > 0 {
            count += (grid[y - 1][x + 1] == target) as usize;
        }
        count += (grid[y][x + 1] == target) as usize;
        if y < h - 1 {
            count += (grid[y + 1][x + 1] == target) as usize;
        }
    }
    count
}

fn main() {
    let input = include_str!("input/day_18.txt");

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let w = grid.len();
    let h = grid[0].len();

    for _round in 0..10 {
        let old = grid.clone();
        for x in 0..w {
            for y in 0..h {
                match old[y][x] {
                    '.' => {
                        if count_grid(&old, x, y, '|') >= 3 {
                            grid[y][x] = '|';
                        }
                    }
                    '|' => {
                        if count_grid(&old, x, y, '#') >= 3 {
                            grid[y][x] = '#';
                        }
                    }
                    '#' => {
                        if count_grid(&old, x, y, '#') == 0 || count_grid(&old, x, y, '|') == 0 {
                            grid[y][x] = '.';
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        for y in 0..h {
            for x in 0..w {
                print!("{}", grid[y][x]);
            }
            println!();
        }
        println!();
    }
    let mut lum = 0;
    let mut wood = 0;
    for y in 0..h {
        for x in 0..w {
            match grid[y][x] {
                '#' => lum += 1,
                '|' => wood += 1,
                _ => {}
            }
        }
    }
    println!("{}", wood);
    println!("{}", lum);
    println!("{}", wood * lum);
}
