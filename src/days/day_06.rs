use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");
    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{}, {}", usize, usize).unwrap())
        .to_vec();

    let count = (0..1000)
        .into_par_iter()
        .map(|y| {
            (0..1000)
                .into_par_iter()
                .filter(|x| parsed.iter().map(|&p| manhattan((*x, y), p)).sum::<usize>() < 10000)
                .count()
        })
        .sum::<usize>();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");
    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{}, {}", usize, usize).unwrap())
        .to_vec();

    fn find_closest(p: Point, points: &[Point]) -> Option<usize> {
        let mut min_dist = usize::MAX;
        let mut second_min = usize::MAX;
        let mut min = points.len();
        for (i, p2) in points.iter().enumerate() {
            let dist = manhattan(p, *p2);
            if dist < min_dist {
                second_min = min_dist;
                min_dist = dist;
                min = i;
            } else if dist < second_min {
                second_min = dist;
            }
        }
        if min_dist != second_min {
            Some(min)
        } else {
            None
        }
    }

    let mut counts = vec![0; parsed.len()];
    for y in 1..999 {
        for x in 1..999 {
            if let Some(c) = find_closest((x, y), &parsed) {
                counts[c] += 1;
            }
        }
    }
    for i in 0..1000 {
        for p in [(i, 0), (i, 999), (0, i), (999, i)] {
            if let Some(c) = find_closest(p, &parsed) {
                counts[c] = 0;
            }
        }
    }

    let max = counts.iter().max().unwrap();

    pv!(max);
}
