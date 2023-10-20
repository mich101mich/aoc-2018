#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_13;
}
use days::day_13;

fn main() {
    day_13::part_one();
}
