use crate::utils::*;

const ORDER: [Dir; 4] = [Dir::Up, Dir::Left, Dir::Right, Dir::Down];

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let mut grid = char_grid(input);
    let neighborhood = grid.manhattan();

    let mut elves = HashMap::new();
    let mut goblins = HashMap::new();
    for (p, v) in grid.grid_iter_mut_index() {
        match *v {
            'E' => {
                elves.insert(p, 200);
                *v = '.';
            }
            'G' => {
                goblins.insert(p, 200);
                *v = '.';
            }
            _ => (),
        }
    }

    'power_loop: for power in 4..=200 {
        let mut grid = grid.clone();
        let mut elves = elves.clone();
        let mut goblins = goblins.clone();

        let mut turn = 0;
        'outer: loop {
            let mut entities = elves
                .keys()
                .map(|p| (*p, true))
                .chain(goblins.keys().map(|p| (*p, false)))
                .to_vec();
            entities.sort_by(|(a, _), (b, _)| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
            let mut killed = HashSet::<Point>::new();
            for (pos, is_elf) in entities.iter_mut() {
                if killed.contains(pos) {
                    continue;
                }
                let (own_faction, other_faction) = if *is_elf {
                    (&mut elves, &mut goblins)
                } else {
                    (&mut goblins, &mut elves)
                };

                if other_faction.is_empty() {
                    break 'outer;
                }

                if !ORDER
                    .iter()
                    .map(|d| *pos + *d)
                    .any(|p| other_faction.contains_key(&p))
                {
                    let open_spots = other_faction
                        .keys()
                        .flat_map(|p| neighborhood.get_all_neighbors(*p))
                        .filter(|p| {
                            grid[*p] == '.'
                                && !own_faction.contains_key(p)
                                && !other_faction.contains_key(p)
                        })
                        .to_vec();

                    let path = ORDER
                        .iter()
                        .map(|d| *pos + *d)
                        .filter(|p| grid[*p] == '.' && !own_faction.contains_key(p))
                        .flat_map(|pos| {
                            dijkstra_search(
                                |pos, out| {
                                    ORDER
                                        .iter()
                                        .map(|d| pos + *d)
                                        .filter(|p| {
                                            grid[*p] == '.'
                                                && !own_faction.contains_key(p)
                                                && !other_faction.contains_key(p)
                                        })
                                        .for_each(|p| out.push(p));
                                },
                                pos,
                                &open_spots,
                            )
                            .into_iter()
                            .map(move |(goal, path)| (pos, goal, path))
                        })
                        .min_by(|(pos_a, goal_a, path_a), (pos_b, goal_b, path_b)| {
                            path_a
                                .cost
                                .cmp(&path_b.cost)
                                .then(goal_a.y.cmp(&goal_b.y))
                                .then(goal_a.x.cmp(&goal_b.x))
                                .then(pos_a.y.cmp(&pos_b.y))
                                .then(pos_a.x.cmp(&pos_b.x))
                        });
                    if let Some((next, _, _)) = path {
                        let hits = own_faction.remove(pos).unwrap();
                        own_faction.insert(next, hits);
                        *pos = next;
                    }
                }

                if let Some(target) = ORDER
                    .iter()
                    .map(|d| *pos + *d)
                    .filter(|p| other_faction.contains_key(p))
                    .min_by_key(|p| other_faction[p])
                {
                    let hits = other_faction.get_mut(&target).unwrap();
                    let power = if *is_elf { power } else { 3 };
                    if *hits > power {
                        *hits -= power;
                    } else {
                        if !*is_elf {
                            continue 'power_loop;
                        }
                        other_faction.remove(&target);
                        killed.insert(target);
                    }
                }
            }
            turn += 1;
        }
        let result = turn * elves.values().chain(goblins.values()).sum::<usize>();
        pv!(result);
        return;
    }
    panic!("No solution found");
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let mut grid = char_grid(input);
    let neighborhood = grid.manhattan();

    let mut elves = HashMap::new();
    let mut goblins = HashMap::new();
    for (p, v) in grid.grid_iter_mut_index() {
        match *v {
            'E' => {
                elves.insert(p, 200);
                *v = '.';
            }
            'G' => {
                goblins.insert(p, 200);
                *v = '.';
            }
            _ => (),
        }
    }

    let mut turn = 0;
    'outer: loop {
        let mut entities = elves
            .keys()
            .map(|p| (*p, true))
            .chain(goblins.keys().map(|p| (*p, false)))
            .to_vec();
        entities.sort_by(|(a, _), (b, _)| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        let mut killed = HashSet::<Point>::new();
        for (pos, is_elf) in entities.iter_mut() {
            if killed.contains(pos) {
                continue;
            }
            let (own_faction, other_faction) = if *is_elf {
                (&mut elves, &mut goblins)
            } else {
                (&mut goblins, &mut elves)
            };

            if other_faction.is_empty() {
                break 'outer;
            }

            if !ORDER
                .iter()
                .map(|d| *pos + *d)
                .any(|p| other_faction.contains_key(&p))
            {
                let open_spots = other_faction
                    .keys()
                    .flat_map(|p| neighborhood.get_all_neighbors(*p))
                    .filter(|p| {
                        grid[*p] == '.'
                            && !own_faction.contains_key(p)
                            && !other_faction.contains_key(p)
                    })
                    .to_vec();

                let path = ORDER
                    .iter()
                    .map(|d| *pos + *d)
                    .filter(|p| grid[*p] == '.' && !own_faction.contains_key(p))
                    .flat_map(|pos| {
                        dijkstra_search(
                            |pos, out| {
                                ORDER
                                    .iter()
                                    .map(|d| pos + *d)
                                    .filter(|p| {
                                        grid[*p] == '.'
                                            && !own_faction.contains_key(p)
                                            && !other_faction.contains_key(p)
                                    })
                                    .for_each(|p| out.push(p));
                            },
                            pos,
                            &open_spots,
                        )
                        .into_iter()
                        .map(move |(goal, path)| (pos, goal, path))
                    })
                    .min_by(|(pos_a, goal_a, path_a), (pos_b, goal_b, path_b)| {
                        path_a
                            .cost
                            .cmp(&path_b.cost)
                            .then(goal_a.y.cmp(&goal_b.y))
                            .then(goal_a.x.cmp(&goal_b.x))
                            .then(pos_a.y.cmp(&pos_b.y))
                            .then(pos_a.x.cmp(&pos_b.x))
                    });
                if let Some((next, _, _)) = path {
                    let hits = own_faction.remove(pos).unwrap();
                    own_faction.insert(next, hits);
                    *pos = next;
                }
            }

            if let Some(target) = ORDER
                .iter()
                .map(|d| *pos + *d)
                .filter(|p| other_faction.contains_key(p))
                .min_by_key(|p| other_faction[p])
            {
                let hits = other_faction.get_mut(&target).unwrap();
                if *hits > 3 {
                    *hits -= 3;
                } else {
                    other_faction.remove(&target);
                    killed.insert(target);
                }
            }
        }
        turn += 1;
    }
    let result = turn * elves.values().chain(goblins.values()).sum::<usize>();
    pv!(result);
}
