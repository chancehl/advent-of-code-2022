// TODO: figure out a way to sort these by number rather than alphabetically
mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;

use std::fs;

// use day_one::problem::part_one::calorie_counting;
// use day_one::problem::part_two::calorie_counting;
// use day_two::problem::part_one::rock_paper_scissors;
// use day_two::problem::part_two::rock_paper_scissors;
// use day_three::problem::part_one::rucksack_reorganization;
// use day_three::problem::part_two::rucksack_reorganization;
// use day_four::problem::part_one::camp_cleanup;
// use day_four::problem::part_two::camp_cleanup;
// use day_five::problem::part_one::supply_stacks;
// use day_five::problem::part_two::supply_stacks;
use day_six::problem::tuning_trouble; // Note: this one has a shared solution for part one and two

fn main() {
    let input = fs::read_to_string("./src/day_six/input.txt").unwrap();

    let end_index = tuning_trouble(&input, 4).unwrap();

    println!("end_index = {:?}", end_index);
}
