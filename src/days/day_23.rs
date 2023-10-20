use crate::utils::*;

const ORIGIN: Point3DI = p3(0, 0, 0);

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ManhattanSphere {
        center: Point3DI,
        radius: isize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct AABB {
        min: Point3DI,
        max: Point3DI,
        count: usize,
    }

    impl AABB {
        fn center(&self) -> Point3DI {
            self.min + (self.max - self.min) / 2
        }
        fn side_length(&self, axis: usize) -> isize {
            self.max[axis] - self.min[axis]
        }
        fn touches(&self, sphere: &ManhattanSphere) -> bool {
            let mut dist = 0;
            for i in 0..3 {
                if sphere.center[i] < self.min[i] {
                    dist += self.min[i] - sphere.center[i];
                } else if sphere.center[i] > self.max[i] {
                    dist += sphere.center[i] - self.max[i];
                }
            }
            dist <= sphere.radius
        }
        fn split_at(&self, axis: usize, spheres: &[ManhattanSphere]) -> [AABB; 2] {
            let mut children = [*self, *self];
            children[0].max[axis] = self.center()[axis];
            children[1].min[axis] = self.center()[axis] + 1;

            children[0].count = 0;
            children[1].count = 0;
            for sphere in spheres {
                for child in &mut children {
                    if child.touches(sphere) {
                        child.count += 1;
                    }
                }
            }
            if children[1].count > children[0].count {
                children.swap(0, 1);
            }
            children
        }
    }
    impl Ord for AABB {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.count.cmp(&other.count)
        }
    }
    impl PartialOrd for AABB {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut bots = input
        .lines()
        .map(|l| sscanf!(l, "pos=<{0},{0},{0}>, r={0}", isize).unwrap())
        .map(|(x, y, z, r)| ManhattanSphere {
            center: p3(x, y, z),
            radius: r,
        })
        .to_vec();

    let total_bb = {
        let mut min = p3(isize::MAX, isize::MAX, isize::MAX);
        let mut max = p3(isize::MIN, isize::MIN, isize::MIN);
        for bot in &bots {
            min = min.cwise_min(bot.center - p3(bot.radius, bot.radius, bot.radius));
            max = max.cwise_max(bot.center + p3(bot.radius, bot.radius, bot.radius));
        }
        AABB {
            min,
            max,
            count: bots.len(),
        }
    };

    let mut best_count = 0;
    let mut best_dist = 0;

    let mut queue = std::collections::BinaryHeap::new();
    queue.push(total_bb);
    let mut at_same_level = vec![];
    while let Some(bb) = queue.pop() {
        let axis = (0..3).max_by_key(|&i| bb.side_length(i)).unwrap();

        if bb.side_length(axis) == 0 {
            best_count = bb.count;
            best_dist = bb.min.manhattan(ORIGIN);
            at_same_level = queue.clone().into_vec();
            queue.retain(|bb| bb.count > best_count);
            continue;
        }

        let children = bb.split_at(axis, &bots);
        for child in children {
            if child.count > best_count {
                queue.push(child);
            }
        }
    }

    drop(queue); // make sure we don't accidentally use it

    let mut improvement_sphere = ManhattanSphere {
        center: ORIGIN,
        radius: best_dist - 1,
    };
    at_same_level.retain(|bb| bb.count >= best_count && bb.touches(&improvement_sphere));

    while let Some(bb) = at_same_level.pop() {
        let axis = (0..3).max_by_key(|&i| bb.side_length(i)).unwrap();

        if bb.side_length(axis) == 0 {
            best_dist = bb.min.manhattan(ORIGIN);
            improvement_sphere.radius = best_dist - 1;
            at_same_level.retain(|bb| bb.touches(&improvement_sphere));
            continue;
        }

        let children = bb.split_at(axis, &bots);
        for child in children {
            if child.count >= best_count && child.touches(&improvement_sphere) {
                at_same_level.push(child);
            }
        }
    }

    pv!(best_dist);
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
