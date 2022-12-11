mod day_one;
mod day_three;
mod day_two;

use std::fs;

// use day_one::problem::calorie_counting;
// use day_two::problem::part_one::rock_paper_scissors;
// use day_two::problem::part_two::rock_paper_scissors;
use day_three::problem::rucksack_reorganization;

fn main() {
    let input = fs::read_to_string("./src/day_three/input.txt").unwrap();

    let priority_sum = rucksack_reorganization(&input);

    println!("priority_sum = {:?}", priority_sum);
}
