use crate::utils::*;

type Point4d = [isize; 4];

fn manhattan_4d(a: Point4d, b: Point4d) -> isize {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");

    let points: Vec<Point4d> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|x| [x[0], x[1], x[2], x[3]])
        .to_vec();

    let mut constellations: Vec<Vec<Point4d>> = vec![];
    for &p in &points {
        let suitable = constellations
            .iter()
            .enumerate()
            .filter(|(_, c)| c.iter().any(|p2| manhattan_4d(p, *p2) <= 3))
            .map(|(i, _)| i)
            .to_vec();
        if suitable.is_empty() {
            constellations.push(vec![p]);
        } else if suitable.len() == 1 {
            constellations[suitable[0]].push(p);
        } else {
            let mut new_constellation = vec![p];
            for &i in suitable.iter().rev() {
                new_constellation.extend(constellations.remove(i));
            }
            constellations.push(new_constellation);
        }
    }
    pv!(constellations.len());
}
