use crate::utils::*;

fn move_forward<T>(list: &mut VecDeque<T>) {
    let temp = list.pop_front().unwrap();
    list.push_back(temp);
}
fn move_backward<T>(list: &mut VecDeque<T>) {
    let temp = list.pop_back().unwrap();
    list.push_front(temp);
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let (players, mut last_marble) = sscanf!(
        input,
        "{usize} players; last marble is worth {usize} points"
    )
    .unwrap();

    last_marble *= 100;

    let mut scores = vec![0; players];
    let mut current_player = 0;
    let mut marbles = VecDeque::new();
    marbles.push_back(0);
    for next_marble in 1..=last_marble {
        if next_marble % 23 == 0 {
            for _ in 0..7 {
                move_backward(&mut marbles);
            }
            scores[current_player] += next_marble + marbles.pop_front().unwrap();
        } else {
            move_forward(&mut marbles);
            move_forward(&mut marbles);
            marbles.push_front(next_marble);
        }
        current_player = (current_player + 1) % players;
    }

    pv!(scores.iter().max().unwrap());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let (players, last_marble) = sscanf!(
        input,
        "{usize} players; last marble is worth {usize} points"
    )
    .unwrap();

    let mut scores = vec![0; players];
    let mut current_player = 0;
    let mut marbles = VecDeque::new();
    marbles.push_back(0);
    for next_marble in 1..=last_marble {
        if next_marble % 23 == 0 {
            for _ in 0..7 {
                move_backward(&mut marbles);
            }
            scores[current_player] += next_marble + marbles.pop_front().unwrap();
        } else {
            move_forward(&mut marbles);
            move_forward(&mut marbles);
            marbles.push_front(next_marble);
        }
        current_player = (current_player + 1) % players;
    }

    pv!(scores.iter().max().unwrap());
}
