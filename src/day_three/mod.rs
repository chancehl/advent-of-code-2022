#[allow(dead_code)]
pub mod problem {
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

    pub mod part_one {
        use super::calculate_priority;

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
    }

    pub mod part_two {
        use crate::day_three::problem::calculate_priority;

        /// Determines the "priority sum" of items in a bunch of rucksacks that were very disorganized
        /// "rucksacks" are grouped into chunks of 3 and they will all share one single
        /// letter between them. a priority is then calculated for each group.
        /// "priority" = the shared letter index
        pub fn rucksack_reorganization(input: &str) -> u32 {
            let mut sum_of_priorities: u32 = 0;

            let all_rucksacks = input.split("\n").collect::<Vec<&str>>();

            let rucksack_groups: Vec<&[&str]> = all_rucksacks.chunks(3).collect();

            for group in rucksack_groups {
                let common_item = find_common_item(group)
                    .expect("Could not locate common item for rucksack group");

                let priority = calculate_priority(common_item)
                    .expect("Could not calculate prioriy for common item");

                sum_of_priorities = priority + sum_of_priorities;
            }

            sum_of_priorities
        }

        /// Finds a common item between 3 sets of of rucksacks
        pub fn find_common_item(rucksack_group: &[&str]) -> Option<char> {
            if let [a, b, c] = rucksack_group {
                for a_char in a.chars() {
                    let mut b_match = false;
                    let mut c_match = false;

                    'b_loop: for b_char in b.chars() {
                        if a_char == b_char {
                            b_match = true;

                            break 'b_loop;
                        }
                    }

                    'c_loop: for c_char in c.chars() {
                        if a_char == c_char && b_match {
                            c_match = true;

                            break 'c_loop;
                        }
                    }

                    if b_match && c_match {
                        return Some(a_char);
                    }
                }
            }

            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::problem::{calculate_priority, part_one, part_two};

    #[test]
    pub fn calculate_priority_test() {
        assert_eq!(calculate_priority('c').unwrap(), 3);
        assert_eq!(calculate_priority('h').unwrap(), 8);
        assert_eq!(calculate_priority('a').unwrap(), 1);
        assert_eq!(calculate_priority('N').unwrap(), 40);
        assert_eq!(calculate_priority('C').unwrap(), 29);
        assert_eq!(calculate_priority('E').unwrap(), 31);
    }

    #[test]
    pub fn part_one_find_common_item_test() {
        assert_eq!(part_one::find_common_item("abCCde").unwrap(), 'C');
        assert_eq!(part_one::find_common_item("ohRemLkzhO").unwrap(), 'h');
        assert_eq!(part_one::find_common_item("IAEXsliA").unwrap(), 'A');
        assert_eq!(part_one::find_common_item("weNasrEmliAN").unwrap(), 'N');
        assert_eq!(part_one::find_common_item("oeRxDcLEcNrX").unwrap(), 'c');
        assert_eq!(part_one::find_common_item("ee").unwrap(), 'e'); // I gave up generating fun strings here
    }

    #[test]
    pub fn part_two_find_common_item_test() {
        assert_eq!(
            part_two::find_common_item(&vec![
                "abcdefghijkl",
                "cmopqrstuvwxyz",
                "ABcDEFGHIJKLMNOPQRSTUVWXYZ",
            ])
            .unwrap(),
            'c'
        );

        assert_eq!(
            part_two::find_common_item(&vec!["h", "hrezdr", "hZSDREds",]).unwrap(),
            'h'
        );

        assert_eq!(
            part_two::find_common_item(&vec!["LersDFT", "cmopqLz", "ABCDEFGHIJKLMNOPQRSTUVWXYZ",])
                .unwrap(),
            'L'
        );
    }

    #[test]
    pub fn part_one_rucksack_organization_test() {
        // happy path
        assert_eq!(part_one::rucksack_reorganization("abcxya"), 1);
        assert_eq!(part_one::rucksack_reorganization("dzjoez"), 26);
        assert_eq!(part_one::rucksack_reorganization("ZyxbcZ"), 52);

        // mixed cases
        assert_eq!(part_one::rucksack_reorganization("abcdefgABCDeFGH"), 5);

        // shared capital
        assert_eq!(part_one::rucksack_reorganization("XYZOWUxkjuX"), 50);

        // multiline
        assert_eq!(
            part_one::rucksack_reorganization("abca\nxyzkjz\nauerKJaUR"),
            28
        );
    }

    #[test]
    pub fn part_two_rucksack_organization_test() {
        assert_eq!(part_two::rucksack_reorganization("abc\ndaf\najk"), 1); // a
        assert_eq!(
            part_two::rucksack_reorganization(
                "lDDWVvlVVQfDMlWjGJTRjQCgTGLCLj\nZLZpwzLBhwZhSLBsjntGCtgJRjbnJgSG\nqppdZzzsdsmZphrNsNwZhllDHLcVVDWFPvFWcWdFlv"
            ),
            38
        )
    }
}
