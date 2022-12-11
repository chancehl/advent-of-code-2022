pub mod problem {
    /// Determines the "priority sum" of items in a bunch of rucksacks that were very disorganized
    /// "rucksacks" are strings of even length who have exactly one shared
    /// character between the two sides when the string is split in half
    /// "priority sum" is the sum of alphabetically sorted character indices e.g. a = 1, b = 2, c = 3 ... z = 26 (note: this char range is a-Z so the final index is 52 rather than 26)
    pub fn rucksack_reorganization(input: &str) -> u32 {
        let mut sum_of_priorities: u32 = 0;
        let rucksacks = parse_rucksacks(input);

        for rucksack in rucksacks {
            let common_item = find_common_item(rucksack)
                .expect("Invalid input. Missing matching char in both halves of rucksack.");

            let item_priority =
                calculate_priority(common_item).expect("Could not calculate item priority");

            sum_of_priorities = sum_of_priorities + item_priority;
        }

        sum_of_priorities
    }

    /// Converts a new line separated input into rucksacks
    /// "rucksacks" are strings of even length who have exactly one shared
    /// character between the two sides when the string is split in half
    pub fn parse_rucksacks(input: &str) -> Vec<&str> {
        input.split("\n").collect::<Vec<&str>>()
    }

    /// Finds the common character between two halves of a string
    pub fn find_common_item(rucksack: &str) -> Option<char> {
        let len = rucksack.len();

        let first_half = &rucksack[0..(len / 2)];
        let second_half = &rucksack[(len / 2)..];

        for char_a in first_half.chars() {
            for char_b in second_half.chars() {
                if char_a == char_b {
                    return Some(char_a);
                }

                continue;
            }
        }

        return None;
    }

    /// Calculates a priority based on an item (character)
    /// priority = the item's position within the alphabet (a-z = 1-26, A-Z = 27-52)
    pub fn calculate_priority(item: char) -> Option<u32> {
        let all_items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

        let mut index = 1;

        for char in all_items {
            if char == item {
                return Some(index);
            } else {
                index = index + 1;
            }
        }

        return None;
    }
}
