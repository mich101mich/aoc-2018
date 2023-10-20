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
            if let Ok((x, y1, y2)) = sscanf!(l, "x={usize}, y={usize}..{usize}") {
                (x..=x, y1..=y2)
            } else {
                let (y, x1, x2) = sscanf!(l, "y={usize}, x={usize}..{usize}").unwrap();
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

    let mut grid = Grid::new_clone(p2(w, h), Tile::Empty);
    for (rx, ry) in parsed {
        for x in rx {
            for y in ry.clone() {
                grid[p2(x, y)] = Tile::Wall;
            }
        }
    }
    let mut flowing = vec![p2(500, 0)];
    let mut still_water = 0;
    grid[flowing[0]] = Tile::FlowingWater;

    while let Some(p) = flowing.pop() {
        if p.y == h - 1 {
            continue;
        }
        if grid[p] != Tile::FlowingWater {
            continue;
        }
        match grid[p + Dir::Down] {
            Tile::Empty => {
                flowing.push(p);
                for pt in (p.y + 1..h).map(|y| p2(p.x, y)) {
                    let tile = &mut grid[pt];
                    if *tile != Tile::Empty {
                        break;
                    }
                    *tile = Tile::FlowingWater;
                    flowing.push(pt);
                }
            }
            t if t.is_solid() => {
                let mut left_closed = false;
                let mut right_closed = false;

                let mut start = p;
                while start.x > 0 {
                    if !grid[start + Dir::Down].is_solid() {
                        grid[start] = Tile::FlowingWater;
                        flowing.push(start);
                        break;
                    } else if grid[start + Dir::Left] == Tile::Wall {
                        left_closed = true;
                        break;
                    }
                    start += Dir::Left;
                }
                let mut end = p;
                while end.x < w - 1 {
                    if !grid[end + Dir::Down].is_solid() {
                        grid[end] = Tile::FlowingWater;
                        flowing.push(end);
                        break;
                    } else if grid[end + Dir::Right] == Tile::Wall {
                        right_closed = true;
                        break;
                    }
                    end += Dir::Right;
                }

                if left_closed && right_closed {
                    grid[p.y][start.x..=end.x].fill(Tile::StillWater);
                    still_water += end.x - start.x + 1;
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
            if let Ok((x, y1, y2)) = sscanf!(l, "x={usize}, y={usize}..{usize}") {
                (x..=x, y1..=y2)
            } else {
                let (y, x1, x2) = sscanf!(l, "y={usize}, x={usize}..{usize}").unwrap();
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

    let mut grid = Grid::new_clone(p2(w, h), Tile::Empty);
    for (rx, ry) in parsed {
        for x in rx {
            for y in ry.clone() {
                grid[p2(x, y)] = Tile::Wall;
            }
        }
    }
    let mut flowing = HashSet::<Point>::new();
    flowing.insert(p2(500, 0));
    grid[p2(500usize, 0)] = Tile::FlowingWater;

    let mut change = true;
    while change {
        change = false;
        let mut new_flowing = HashSet::new();
        let mut remove_flowing = HashSet::new();
        for p in flowing.iter().copied() {
            if p.y == h - 1 {
                continue;
            }
            if grid[p] != Tile::FlowingWater {
                remove_flowing.insert(p);
                continue;
            }
            match grid[p + Dir::Down] {
                Tile::Empty => {
                    for pt in (p.y + 1..h).map(|y| p2(p.x, y)) {
                        let tile = &mut grid[pt];
                        if *tile != Tile::Empty {
                            break;
                        }
                        *tile = Tile::FlowingWater;
                        new_flowing.insert(pt);
                    }
                    change = true;
                }
                t if t.is_solid() => {
                    let mut start = p;
                    while start.x > 0
                        && !grid[start + Dir::Left].is_solid()
                        && grid[start + Dir::Down].is_solid()
                    {
                        start += Dir::Left;
                    }
                    let mut end = p;
                    while end.x < w - 1
                        && !grid[end + Dir::Right].is_solid()
                        && grid[end + Dir::Down].is_solid()
                    {
                        end += Dir::Right;
                    }

                    if grid[start + Dir::Left] == Tile::Wall && grid[end + Dir::Right] == Tile::Wall
                    {
                        grid[p.y][start.x..=end.x].fill(Tile::StillWater);
                    } else {
                        grid[p.y][start.x..=end.x].fill(Tile::FlowingWater);
                        new_flowing.insert(start);
                        new_flowing.insert(end);
                    }
                    remove_flowing.insert(p);
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
        .filter(|(p, _)| p.y >= min_y)
        .filter(|(_, t)| matches!(**t, Tile::StillWater | Tile::FlowingWater))
        .count();
    pv!(cnt);
}
