use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut bots = input
        .lines()
        .map(|l| sscanf!(l, "pos=<{0},{0},{0}>, r={0}", isize).unwrap())
        .map(|(x, y, z, r)| (p3(x, y, z), r))
        .to_vec();

    fn count_in_range(p: Point3DI, points: &[(Point3DI, isize)]) -> usize {
        points
            .iter()
            .filter(|&&(p2, r)| p.manhattan(p2) <= r)
            .count()
    }
    let origin = p3(0, 0, 0);

    let mut min = bots
        .iter()
        .fold(p3(isize::MAX, isize::MAX, isize::MAX), |a, (p, _)| {
            p3(a.x.min(p.x), a.y.min(p.y), a.z.min(p.z))
        });
    let mut max = bots
        .iter()
        .fold(p3(isize::MIN, isize::MIN, isize::MIN), |a, (p, _)| {
            p3(a.x.max(p.x), a.y.max(p.y), a.z.max(p.z))
        });

    let mut scale = 10_000_000;
    min = min / scale - p3(1, 1, 1);
    max = max / scale + p3(1, 1, 1);

    let mut max_cell = p3(0, 0, 0);
    let mut max_count = 0;

    while scale > 0 {
        let add = (scale > 1) as isize;
        let bots = bots
            .iter()
            .map(|(p, r)| {
                (
                    p3(
                        (p.x as f32 / scale as f32).floor() as isize,
                        (p.y as f32 / scale as f32).floor() as isize,
                        (p.z as f32 / scale as f32).floor() as isize,
                    ),
                    (*r as f32 / scale as f32).ceil() as isize + add,
                )
            })
            .to_vec();

        max_count = 0;
        max_cell = p3(0, 0, 0);
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let p = p3(x, y, z);
                    let count = count_in_range(p, &bots);
                    if count > max_count {
                        max_count = count;
                        max_cell = p;
                    }
                }
            }
        }
        min = max_cell * 10 - p3(5, 5, 5);
        max = (max_cell + p3(1, 1, 1)) * 10 + p3(5, 5, 5);
        scale /= 10;
    }
    let mut best_dist = max_cell.manhattan(origin);
    'outer: loop {
        for dx in -10..=10 {
            for dy in -10..=10 {
                for dz in -10..=10 {
                    let p = max_cell + p3(dx, dy, dz);
                    let count = count_in_range(p, &bots);
                    match count.cmp(&max_count) {
                        Ordering::Greater => {
                            max_count = count;
                            max_cell = p;
                            best_dist = max_cell.manhattan(origin);
                            continue 'outer;
                        }
                        Ordering::Equal => {
                            let dist = p.manhattan(origin);
                            if dist < best_dist {
                                best_dist = dist;
                                max_cell = p;
                                continue 'outer;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        break;
    }
    let count = count_in_range(max_cell, &bots);
    let dist = max_cell.manhattan(origin);
    pv!(dist);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "pos=<{0},{0},{0}>, r={0}", isize).unwrap())
        .map(|(x, y, z, r)| (p3(x, y, z), r))
        .to_vec();

    let largest_radius = parsed.iter().max_by_key(|(_, r)| *r).unwrap();
    let cnt = parsed
        .iter()
        .filter(|(p, _)| p.manhattan(largest_radius.0) <= largest_radius.1)
        .count();
    pv!(cnt);
}
