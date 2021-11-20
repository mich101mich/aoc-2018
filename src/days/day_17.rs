use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    FlowingWater,
    StillWater,
}
impl Tile {
    pub fn is_solid(&self) -> bool {
        matches!(self, Tile::Wall | Tile::StillWater)
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let parsed = input
        .lines()
        .map(|l| {
            if let Some((x, y1, y2)) = scanf!(l, "x={}, y={}..{}", usize, usize, usize) {
                (x..=x, y1..=y2)
            } else {
                let (y, x1, x2) = scanf!(l, "y={}, x={}..{}", usize, usize, usize).unwrap();
                (x1..=x2, y..=y)
            }
        })
        .to_vec();

    let (mut w, mut h, min_y) = parsed
        .iter()
        .fold((0, 0, usize::MAX), |(x, y, min_y), (rx, ry)| {
            (x.max(*rx.end()), y.max(*ry.end()), min_y.min(*ry.start()))
        });
    w += 10;
    h += 1;

    let mut grid = Grid::new_clone((w, h), Tile::Empty);
    for (rx, ry) in parsed {
        for x in rx {
            for y in ry.clone() {
                grid[(x, y)] = Tile::Wall;
            }
        }
    }
    let mut flowing = vec![(500, 0)];
    let mut still_water = 0;
    grid[flowing[0]] = Tile::FlowingWater;

    while let Some((x, y)) = flowing.pop() {
        if y == h - 1 {
            continue;
        }
        if grid[(x, y)] != Tile::FlowingWater {
            continue;
        }
        match grid[(x, y + 1)] {
            Tile::Empty => {
                flowing.push((x, y));
                for ty in y + 1..h {
                    let tile = &mut grid[(x, ty)];
                    if *tile != Tile::Empty {
                        break;
                    }
                    *tile = Tile::FlowingWater;
                    flowing.push((x, ty));
                }
            }
            t if t.is_solid() => {
                let mut left_closed = false;
                let mut right_closed = false;

                let mut start_x = x;
                while start_x > 0 {
                    if !grid[(start_x, y + 1)].is_solid() {
                        grid[(start_x, y)] = Tile::FlowingWater;
                        flowing.push((start_x, y));
                        break;
                    } else if grid[(start_x - 1, y)] == Tile::Wall {
                        left_closed = true;
                        break;
                    }
                    start_x -= 1;
                }
                let mut end_x = x;
                while end_x < w - 1 {
                    if !grid[(end_x, y + 1)].is_solid() {
                        grid[(end_x, y)] = Tile::FlowingWater;
                        flowing.push((end_x, y));
                        break;
                    } else if grid[(end_x + 1, y)] == Tile::Wall {
                        right_closed = true;
                        break;
                    }
                    end_x += 1;
                }

                if left_closed && right_closed {
                    grid[y][start_x..=end_x].fill(Tile::StillWater);
                    still_water += end_x - start_x + 1;
                }
            }
            _ => {}
        }
    }

    pv!(still_water);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let parsed = input
        .lines()
        .map(|l| {
            if let Some((x, y1, y2)) = scanf!(l, "x={}, y={}..{}", usize, usize, usize) {
                (x..=x, y1..=y2)
            } else {
                let (y, x1, x2) = scanf!(l, "y={}, x={}..{}", usize, usize, usize).unwrap();
                (x1..=x2, y..=y)
            }
        })
        .to_vec();

    let (mut w, mut h, min_y) = parsed
        .iter()
        .fold((0, 0, usize::MAX), |(x, y, min_y), (rx, ry)| {
            (x.max(*rx.end()), y.max(*ry.end()), min_y.min(*ry.start()))
        });
    w += 10;
    h += 1;

    let mut grid = Grid::new_clone((w, h), Tile::Empty);
    for (rx, ry) in parsed {
        for x in rx {
            for y in ry.clone() {
                grid[(x, y)] = Tile::Wall;
            }
        }
    }
    let mut flowing = HashSet::<Point>::new();
    flowing.insert((500, 0));
    grid[(500usize, 0)] = Tile::FlowingWater;

    let mut change = true;
    while change {
        change = false;
        let mut new_flowing = HashSet::new();
        let mut remove_flowing = HashSet::new();
        for &(x, y) in flowing.iter() {
            if y == h - 1 {
                continue;
            }
            if grid[(x, y)] != Tile::FlowingWater {
                remove_flowing.insert((x, y));
                continue;
            }
            match grid[(x, y + 1)] {
                Tile::Empty => {
                    for ty in y + 1..h {
                        let tile = &mut grid[(x, ty)];
                        if *tile != Tile::Empty {
                            break;
                        }
                        *tile = Tile::FlowingWater;
                        new_flowing.insert((x, ty));
                    }
                    change = true;
                }
                t if t.is_solid() => {
                    let mut start_x = x;
                    while start_x > 0
                        && !grid[(start_x - 1, y)].is_solid()
                        && grid[(start_x, y + 1)].is_solid()
                    {
                        start_x -= 1;
                    }
                    let mut end_x = x;
                    while end_x < w - 1
                        && !grid[(end_x + 1, y)].is_solid()
                        && grid[(end_x, y + 1)].is_solid()
                    {
                        end_x += 1;
                    }

                    if grid[(start_x - 1, y)] == Tile::Wall && grid[(end_x + 1, y)] == Tile::Wall {
                        for tx in start_x..=end_x {
                            grid[(tx, y)] = Tile::StillWater;
                        }
                    } else {
                        for tx in start_x..=end_x {
                            grid[(tx, y)] = Tile::FlowingWater;
                        }
                        new_flowing.insert((start_x, y));
                        new_flowing.insert((end_x, y));
                    }
                    remove_flowing.insert((x, y));
                    change = true;
                }
                _ => {}
            }
        }
        for p in new_flowing {
            flowing.insert(p);
        }
        for p in remove_flowing {
            flowing.remove(&p);
        }
    }

    let cnt = grid
        .grid_iter_index()
        .filter(|((_, y), _)| *y >= min_y)
        .filter(|(_, t)| matches!(**t, Tile::StillWater | Tile::FlowingWater))
        .count();
    pv!(cnt);
}
