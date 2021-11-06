#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_07;
}
use days::day_07;

fn main() {
    day_07::run();
}
