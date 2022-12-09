mod day_one;

use std::fs;

use day_one::calorie_counting;

fn main() {
    let input = fs::read_to_string("./src/day_one/input.txt").unwrap();

    let max = calorie_counting(&input);

    println!("max = {:?}", max);
}
