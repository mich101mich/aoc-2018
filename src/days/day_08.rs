use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = input.split_whitespace().map(parse_u).to_vec();

    fn node_value(tree: &[usize]) -> (usize, &[usize]) {
        if let ([children, meta], mut tree) = tree.split_at(2) {
            let value = if *children == 0 {
                tree[..*meta].iter().sum::<usize>()
            } else {
                let child_values = (0..*children)
                    .map(|_| {
                        let (v, remainder) = node_value(tree);
                        tree = remainder;
                        v
                    })
                    .to_vec();

                tree[..*meta]
                    .iter()
                    .filter_map(|i| child_values.get(*i - 1))
                    .sum::<usize>()
            };
            (value, &tree[*meta..])
        } else {
            panic!("invalid tree")
        }
    }

    pv!(node_value(&parsed).0);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = input.split_whitespace().map(parse_u).to_vec();

    fn sum_meta(tree: &[usize]) -> (usize, &[usize]) {
        if let ([children, meta], mut tree) = tree.split_at(2) {
            let mut sum = 0;
            for _ in 0..*children {
                let (child_sum, remainder) = sum_meta(tree);
                sum += child_sum;
                tree = remainder
            }
            sum += tree[..*meta].iter().sum::<usize>();
            (sum, &tree[*meta..])
        } else {
            panic!("invalid tree")
        }
    }

    pv!(sum_meta(&parsed).0);
}
