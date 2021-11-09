#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_15;
}
use days::day_15;

fn main() {
    day_15::run();
}
