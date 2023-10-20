use crate::utils::*;

#[derive(FromScanf)]
enum Action {
    #[sscanf(format = "Guard #{} begins shift")]
    BeginsShift(usize),
    #[sscanf(format = "falls asleep")]
    FallsAsleep,
    #[sscanf(format = "wakes up")]
    WakesUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromScanf)]
#[sscanf(format = "[{year}-{month}-{day} {hour}:{minute}]")]
struct TimeStamp {
    year: usize,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{TimeStamp} {Action}").unwrap())
        .to_vec();

    parsed.sort_unstable_by_key(|(ts, _)| *ts);

    let mut guard_id = 0;
    let mut sleep_start = 0;
    let mut total_sleep = HashMap::new();
    let mut minutes_slept = HashMap::new();

    for action in parsed {
        match action.1 {
            Action::BeginsShift(id) => guard_id = id,
            Action::FallsAsleep => {
                sleep_start = action.0.minute;
            }
            Action::WakesUp => {
                let sleep_end = action.0.minute;
                let time = sleep_end - sleep_start;
                *total_sleep.entry(guard_id).or_insert(0) += time as usize;

                let mut minutes = minutes_slept.entry(guard_id).or_insert_with(|| vec![0; 60]);
                for i in sleep_start..sleep_end {
                    minutes[i as usize] += 1;
                }
            }
        }
    }

    let (guard_id, minute, _) = minutes_slept
        .iter()
        .map(|(id, minutes)| {
            let (minute, times) = minutes.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
            (*id, minute, *times)
        })
        .max_by_key(|(_, _, times)| *times)
        .unwrap();

    pv!(guard_id * minute);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{TimeStamp} {Action}").unwrap())
        .to_vec();

    parsed.sort_unstable_by_key(|(ts, _)| *ts);

    let mut guard_id = 0;
    let mut sleep_start = 0;
    let mut total_sleep = HashMap::new();
    let mut minutes_slept = HashMap::new();

    for action in parsed {
        match action.1 {
            Action::BeginsShift(id) => guard_id = id,
            Action::FallsAsleep => {
                sleep_start = action.0.minute;
            }
            Action::WakesUp => {
                let sleep_end = action.0.minute;
                let time = sleep_end - sleep_start;
                *total_sleep.entry(guard_id).or_insert(0) += time as usize;

                let mut minutes = minutes_slept.entry(guard_id).or_insert_with(|| vec![0; 60]);
                for i in sleep_start..sleep_end {
                    minutes[i as usize] += 1;
                }
            }
        }
    }

    let (guard_id, _) = total_sleep.iter().max_by_key(|(_, time)| *time).unwrap();

    let minutes = minutes_slept.get(guard_id).unwrap();
    let (minute, _) = minutes
        .iter()
        .enumerate()
        .max_by_key(|(_, time)| *time)
        .unwrap();

    pv!(guard_id * minute);
}
