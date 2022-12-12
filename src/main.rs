// TODO: figure out a way to sort these by number rather than alphabetically
mod day_four;
mod day_one;
mod day_three;
mod day_two;

use std::fs;

// use day_one::problem::calorie_counting;
// use day_two::problem::part_one::rock_paper_scissors;
// use day_two::problem::part_two::rock_paper_scissors;
// use day_three::problem::part_one::rucksack_reorganization;
// use day_three::problem::part_two::rucksack_reorganization;
// use day_four::problem::part_one::camp_cleanup;
use day_four::problem::part_two::camp_cleanup;

fn main() {
    let input = fs::read_to_string("./src/day_four/input.txt").unwrap();

    let overlapping_ranges = camp_cleanup(&input);

    println!("overlapping_ranges = {:?}", overlapping_ranges);
}
