use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point3d(isize, isize, isize);

impl Point3d {
    fn distance(&self, other: &Point3d) -> isize {
        manhattan_3d_i((self.0, self.1, self.2), (other.0, other.1, other.2))
    }
}

impl RegexRepresentation for Point3d {
    const REGEX: &'static str = r"<-?\d+,-?\d+,-?\d+>";
}
impl std::str::FromStr for Point3d {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('<')
            .and_then(|s| s.strip_suffix('>'))
            .unwrap();
        let mut iter = s.split(',').map(|s| s.parse());
        Ok(Point3d(
            iter.next().unwrap()?,
            iter.next().unwrap()?,
            iter.next().unwrap()?,
        ))
    }
}
impl std::ops::Add<Point3d> for Point3d {
    type Output = Point3d;
    fn add(self, other: Point3d) -> Point3d {
        Point3d(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl std::ops::Sub<Point3d> for Point3d {
    type Output = Point3d;
    fn sub(self, other: Point3d) -> Point3d {
        Point3d(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl std::ops::Mul<isize> for Point3d {
    type Output = Point3d;
    fn mul(self, other: isize) -> Point3d {
        Point3d(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl std::ops::Div<isize> for Point3d {
    type Output = Point3d;
    fn div(self, other: isize) -> Point3d {
        Point3d(self.0 / other, self.1 / other, self.2 / other)
    }
}
impl std::ops::Mul<Point3d> for Point3d {
    type Output = Point3d;
    fn mul(self, other: Point3d) -> Point3d {
        Point3d(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
impl From<(isize, isize, isize)> for Point3d {
    fn from(t: (isize, isize, isize)) -> Self {
        Point3d(t.0, t.1, t.2)
    }
}
impl From<isize> for Point3d {
    fn from(t: isize) -> Self {
        Point3d(t, t, t)
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut bots = input
        .lines()
        .map(|l| scanf!(l, "pos={}, r={}", Point3d, isize).unwrap())
        .to_vec();

    fn count_in_range(p: Point3d, points: &[(Point3d, isize)]) -> usize {
        points.iter().filter(|(p2, r)| p.distance(p2) <= *r).count()
    }
    let origin = Point3d::from(0);

    let mut min = bots.iter().fold(Point3d::from(isize::MAX), |a, (p, _)| {
        Point3d(a.0.min(p.0), a.1.min(p.1), a.2.min(p.2))
    });
    let mut max = bots.iter().fold(Point3d::from(isize::MIN), |a, (p, _)| {
        Point3d(a.0.max(p.0), a.1.max(p.1), a.2.max(p.2))
    });

    let mut scale = 10_000_000;
    min = min / scale - Point3d::from(1);
    max = max / scale + Point3d::from(1);

    let mut max_cell = Point3d::from(0);
    let mut max_count = 0;

    while scale > 0 {
        let add = if scale > 1 { 1 } else { 0 };
        let bots = bots
            .iter()
            .map(|(p, r)| {
                (
                    Point3d(
                        (p.0 as f32 / scale as f32).floor() as isize,
                        (p.1 as f32 / scale as f32).floor() as isize,
                        (p.2 as f32 / scale as f32).floor() as isize,
                    ),
                    (*r as f32 / scale as f32).ceil() as isize + add,
                )
            })
            .to_vec();

        max_count = 0;
        max_cell = Point3d::from(0);
        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                for z in min.2..=max.2 {
                    let p = Point3d(x, y, z);
                    let count = count_in_range(p, &bots);
                    if count > max_count {
                        max_count = count;
                        max_cell = p;
                    }
                }
            }
        }
        min = max_cell * 10 - Point3d::from(5);
        max = (max_cell + Point3d::from(1)) * 10 + Point3d::from(5);
        scale /= 10;
    }
    let mut best_dist = max_cell.distance(&origin);
    'outer: loop {
        for dx in -10..=10 {
            for dy in -10..=10 {
                for dz in -10..=10 {
                    let p = max_cell + Point3d(dx, dy, dz);
                    let count = count_in_range(p, &bots);
                    match count.cmp(&max_count) {
                        Ordering::Greater => {
                            max_count = count;
                            max_cell = p;
                            best_dist = max_cell.distance(&origin);
                            continue 'outer;
                        }
                        Ordering::Equal => {
                            let dist = p.distance(&origin);
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
    let dist = max_cell.distance(&origin);
    pv!(dist);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "pos={}, r={}", Point3d, isize).unwrap())
        .to_vec();

    let largest_radius = parsed.iter().max_by_key(|(_, r)| *r).unwrap();
    let cnt = parsed
        .iter()
        .filter(|(p, _)| p.distance(&largest_radius.0) <= largest_radius.1)
        .count();
    pv!(cnt);
}
