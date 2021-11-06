DAY=$1
echo "use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!(\"../input/$DAY.txt\");
    // let input = "";
    
    let parsed = input
        //.lines()
        //.chars()
        //.map(|l| l.chars().to_vec())
        //.map(parse)
        //.map(|l| scanf!(l, \"{}\", isize).unwrap())
        //.to_vec()
        //.sum::<isize>()
        //.parse::<isize>()
        ;
    
    //pv!(parsed);
    
}" > src/days/day_$DAY.rs

echo "#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_$DAY;
}
use days::day_$DAY;

fn main() {
    day_$DAY::run();
}" > src/main.rs

touch src/input/$DAY.txt

if [ -n $WSL_DISTRO_NAME ]; then
    cmd.exe /c code src/days/day_$DAY.rs src/input/$DAY.txt
else
    code src/days/day_$DAY.rs src/input/$DAY.txt
fi
