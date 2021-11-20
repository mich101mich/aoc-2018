use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let mut grid = char_grid(input);
    let mut next_grid = grid.clone();
    let neighborhood = grid.moore();

    let final_grid = detect_loop(1000000000, || {
        next_grid.grid_iter_mut_index().for_each(|(pos, v)| {
            let mut trees = 0;
            let mut lumbers = 0;
            for p in neighborhood.get_all_neighbors(pos) {
                match grid[p] {
                    '|' => trees += 1,
                    '#' => lumbers += 1,
                    _ => {}
                }
            }
            *v = match grid[pos] {
                '.' if trees >= 3 => '|',
                '|' if lumbers >= 3 => '#',
                '#' if trees == 0 || lumbers == 0 => '.',
                c => c,
            }
        });
        std::mem::swap(&mut grid, &mut next_grid);
        grid.clone()
    });

    let mut trees = 0;
    let mut lumbers = 0;
    for v in final_grid.grid_iter() {
        match v {
            '|' => trees += 1,
            '#' => lumbers += 1,
            _ => {}
        }
    }
    pv!(trees * lumbers);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let mut grid = char_grid(input);
    let mut next_grid = grid.clone();
    let neighborhood = grid.moore();

    for _ in 0..10 {
        next_grid.grid_iter_mut_index().for_each(|(pos, v)| {
            let mut trees = 0;
            let mut lumbers = 0;
            for p in neighborhood.get_all_neighbors(pos) {
                match grid[p] {
                    '|' => trees += 1,
                    '#' => lumbers += 1,
                    _ => {}
                }
            }
            *v = match grid[pos] {
                '.' if trees >= 3 => '|',
                '|' if lumbers >= 3 => '#',
                '#' if trees == 0 || lumbers == 0 => '.',
                c => c,
            }
        });
        std::mem::swap(&mut grid, &mut next_grid);
    }
    let mut trees = 0;
    let mut lumbers = 0;
    for v in grid.grid_iter() {
        match v {
            '|' => trees += 1,
            '#' => lumbers += 1,
            _ => {}
        }
    }
    pv!(trees * lumbers);
}
