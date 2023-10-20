#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_23;
}
use days::day_23;

fn main() {
    day_23::run();
}
