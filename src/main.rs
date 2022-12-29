// TODO: figure out a way to sort these by number rather than alphabetically
mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;

mod shared;

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
// use day_six::problem::tuning_trouble; // Note: this one has a shared solution for part one and two
// use day_seven::problem::part_one::no_space_on_device;
// use day_eight::problem::part_one::treetop_tree_house;
// use day_eight::problem::part_two::treetop_tree_house;
// use day_nine::problem::part_one::rope_bridge;
use day_ten::problem::part_one::cathode_ray_tube;

fn main() {
    let input = fs::read_to_string("./src/day_ten/input.txt").unwrap();

    let signal_strength = cathode_ray_tube(&input);

    println!("signal_strength = {:?}", signal_strength);
}
