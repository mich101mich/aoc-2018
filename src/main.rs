#![allow(unused_imports)]
#![allow(clippy::cyclomatic_complexity)]
#![allow(clippy::needless_range_loop)]

use crate::Dir::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

#[macro_use]
mod utils;
use crate::utils::*;

fn get_count(bots: &[((isize, isize, isize), usize)], pos: (isize, isize, isize)) -> usize {
    bots.par_iter()
        .filter(|&(p, r)| manhattan_3d_i(*p, pos) <= *r)
        .count()
}

fn main() {
    let input = include_str!("input/day_23.txt");

    let bots = input
        .lines()
        .map(|line| {
            let mut split = line[5..].split(">, r=");
            let pos = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| isize::from_str(s).unwrap())
                .collect::<Vec<_>>();
            let radius = usize::from_str(split.next().unwrap()).unwrap();
            ((pos[0], pos[1], pos[2]), radius)
        })
        .collect::<Vec<_>>();

    let mut min_pos = bots[0].0;
    let mut max_pos = bots[0].0;
    for &(p, _) in &bots {
        min_pos.0 = min_pos.0.min(p.0);
        min_pos.1 = min_pos.1.min(p.1);
        min_pos.2 = min_pos.2.min(p.2);
        max_pos.0 = max_pos.0.max(p.0);
        max_pos.1 = max_pos.1.max(p.1);
        max_pos.2 = max_pos.2.max(p.2);
    }

    let mut rng = rand::thread_rng();

    let mut resolution = 1000000;
    let gen_size = 200;

    let mut positions = vec![
        ((0, 0, 0), get_count(&bots, (0, 0, 0))),
        ((15972003, 44657553, 29285970), 977),
    ];
    for _ in 0..gen_size {
        let pos = (
            rng.gen_range(min_pos.0, max_pos.0),
            rng.gen_range(min_pos.1, max_pos.1),
            rng.gen_range(min_pos.2, max_pos.2),
        );
        positions.push((pos, get_count(&bots, pos)));
    }

    while resolution > 0 {
        for _ in 0..(gen_size / 2) {
            let pos = (
                rng.gen_range(min_pos.0, max_pos.0),
                rng.gen_range(min_pos.1, max_pos.1),
                rng.gen_range(min_pos.2, max_pos.2),
            );
            positions.push((pos, get_count(&bots, pos)));
        }
        positions.par_sort_unstable_by_key(|(_, count)| bots.len() - *count);
        pv!(positions[0]);
        positions.truncate(gen_size);
        let lowest = positions[gen_size - 1].1;
        for (p, _) in positions.clone() {
            for dx in -4..=4 {
                for dy in -4..=4 {
                    for dz in -4..=4 {
                        let pos = (
                            p.0 + dx * resolution,
                            p.1 + dy * resolution,
                            p.2 + dz * resolution,
                        );
                        let count = get_count(&bots, pos);
                        if count > lowest {
                            positions.push((pos, count));
                        }
                    }
                }
            }
        }
        resolution = (resolution as f32 * 0.8) as isize;
        pv!(resolution);
    }
    // ((15972003, 44657553, 29285970), 977)

    let best_pos = positions[0].0;

    let mut best_count = 0;
    let mut real_best_pos = best_pos;
    let mut best_dist = manhattan_3d_i(real_best_pos, (0, 0, 0));
    for dx in -50..=50 {
        for dy in -50..=50 {
            for dz in -50..=50 {
                let pos = (best_pos.0 + dx, best_pos.1 + dy, best_pos.2 + dz);
                let count = get_count(&bots, pos);
                if count > best_count
                    || (count == best_count && manhattan_3d_i(pos, (0, 0, 0)) < best_dist)
                {
                    best_count = count;
                    real_best_pos = pos;
                    best_dist = manhattan_3d_i(real_best_pos, (0, 0, 0));
                }
            }
        }
    }
    pv!(real_best_pos == best_pos);
    pv!(real_best_pos);
    pv!(best_count);
    pv!(best_dist); // more than 1047 and 35626375 and 87924130
                    // not enough: best_count = 912 and best_dist = 87924130
}

#[allow(unused)]
fn assembler(input: &str) {
    let (lines, ip_reg) = parse_asm(input);

    let mut registers = [0; 6];

    let mut ip = 0;

    for round in 0_usize.. {
        if ip >= lines.len() {
            println!("stops at {}", round);
            return;
        }

        asm_run(lines[ip], &mut registers);

        ip = registers[ip_reg];
        ip += 1;
    }
}
