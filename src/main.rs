mod day_one;
mod day_two;

use std::fs;

// use day_one::problem::calorie_counting;
// use day_two::problem::part_one::rock_paper_scissors;
use day_two::problem::part_two::rock_paper_scissors;

fn main() {
    let input = fs::read_to_string("./src/day_two/input.txt").unwrap();

    let score = rock_paper_scissors(&input);

    println!("score = {:?}", score);
}
