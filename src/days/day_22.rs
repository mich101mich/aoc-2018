use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");
    //     let input = "depth: 510
    // target: 10,10";

    let mut iter = input.lines();
    let depth = sscanf!(iter.next().unwrap(), "depth: {usize}").unwrap();
    let target = sscanf!(iter.next().unwrap(), "target: {usize},{usize}").unwrap();
    let target = p2(target.0, target.1);

    let mut grid = Grid::<u8>::new_default(p2(target.x * 4, target.y * 2));
    let w = grid.size().x;
    let h = grid.size().y;
    let mut prev_row = vec![0; w];
    for (y, row) in grid.iter_mut().enumerate() {
        let mut prev_val = 0;
        for (x, (v, prev_row)) in row.iter_mut().zip(prev_row.iter_mut()).enumerate() {
            let geo_index = if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else if p2(x, y) == target {
                0
            } else {
                prev_val * *prev_row
            };
            let erosion_level = (geo_index + depth) % 20183;
            *v = (erosion_level % 3) as u8;
            prev_val = erosion_level;
            *prev_row = erosion_level;
        }
    }

    const NEITHER: u8 = 0;
    const TORCH: u8 = 1;
    const CLIMBING_GEAR: u8 = 2;

    let neighbors = grid.manhattan();
    let path = a_star_search(
        |(pos, equipment), out| {
            for p in neighbors.get_all_neighbors(pos) {
                let cannot_use = grid[p];
                if equipment != cannot_use {
                    out.push(((p, equipment), 1));
                }
            }
            let cannot_switch_to = grid[pos];
            for e in 0..3 {
                if e != equipment && e != cannot_switch_to {
                    out.push(((pos, e), 7));
                }
            }
        },
        (p2(0, 0), TORCH),
        (target, TORCH),
        |(p, e)| neighbors.heuristic(p, target) + (e != TORCH) as usize * 7,
    )
    .unwrap();

    pv!(path.cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let mut iter = input.lines();
    let depth = sscanf!(iter.next().unwrap(), "depth: {usize}").unwrap();
    let target = sscanf!(iter.next().unwrap(), "target: {usize},{usize}").unwrap();

    let mut grid = Grid::<u8>::new_default(p2(target.0 + 1, target.1 + 1));
    let w = grid.size().x;
    let h = grid.size().y;
    let mut prev_row = vec![0; w];
    for (y, row) in grid.iter_mut().enumerate() {
        let mut prev_val = 0;
        for (x, (v, prev_row)) in row.iter_mut().zip(prev_row.iter_mut()).enumerate() {
            let geo_index = if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else if (x, y) == target {
                0
            } else {
                prev_val * *prev_row
            };
            let erosion_level = (geo_index + depth) % 20183;
            *v = (erosion_level % 3) as u8;
            prev_val = erosion_level;
            *prev_row = erosion_level;
        }
    }

    let risk_level = grid.grid_iter().map(|v| *v as usize).sum::<usize>();
    pv!(risk_level);
}
