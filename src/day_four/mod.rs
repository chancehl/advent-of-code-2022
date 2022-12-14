#[allow(dead_code)]
pub mod problem {
    use std::ops::RangeInclusive;

    /// Converts a new line separated range into inclusive range objects
    /// example: 1-3,5-6\n4-5,1-2 -> [(1..=3, 5..=6), (4..=5, 1..=2)]
    pub fn parse_section_ranges(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
        input
            .split("\n")
            .map(|l| l.split(",").collect::<Vec<&str>>())
            .map(|group| {
                (
                    convert_section_assignment_to_range(group[0]),
                    convert_section_assignment_to_range(group[1]),
                )
            })
            .collect::<Vec<_>>()
    }

    /// Converts an elf section assignment to an inclusive range
    /// example: 1-3 -> 1..=3
    pub fn convert_section_assignment_to_range(input: &str) -> RangeInclusive<u32> {
        let elements = input
            .split("-")
            .map(|e| e.parse().expect("Could not parse u32 from element"))
            .collect::<Vec<u32>>();

        elements[0]..=elements[1]
    }

    pub mod part_one {
        use super::parse_section_ranges;

        /// Elves have been assigned sections (represented by inclusive ranges e.g. 1..5) to clean up
        /// but before cleaning they have discovered that a bunch of the work has been duplicated
        /// this function determines how many elves have their sections entirely covered by another elf
        pub fn camp_cleanup(input: &str) -> u32 {
            let mut overlapping_ranges: u32 = 0;

            let section_ranges = parse_section_ranges(input);

            for (section_a, section_b) in section_ranges {
                if section_a.contains(&section_b.start()) && section_a.contains(&section_b.end()) {
                    overlapping_ranges = overlapping_ranges + 1;
                } else if section_b.contains(&section_a.start())
                    && section_b.contains(&section_a.end())
                {
                    overlapping_ranges = overlapping_ranges + 1;
                }
            }

            overlapping_ranges
        }
    }

    pub mod part_two {
        use super::parse_section_ranges;

        /// Elves have been assigned sections (represented by inclusive ranges e.g. 1..5) to clean up
        /// but before cleaning they have discovered that a bunch of the work has been duplicated
        /// this function determines how many elves have their sections partially covered by another elf
        pub fn camp_cleanup(input: &str) -> u32 {
            let mut overlapping_ranges: u32 = 0;

            let section_ranges = parse_section_ranges(input);

            for (section_a, section_b) in section_ranges {
                if section_a.contains(&section_b.start()) || section_a.contains(&section_b.end()) {
                    overlapping_ranges = overlapping_ranges + 1;
                } else if section_b.contains(&section_a.start())
                    || section_b.contains(&section_a.end())
                {
                    overlapping_ranges = overlapping_ranges + 1;
                }
            }

            overlapping_ranges
        }
    }
}

#[cfg(test)]
mod tests {
    use super::problem::{
        convert_section_assignment_to_range, parse_section_ranges, part_one, part_two,
    };

    #[test]
    pub fn convert_section_assignment_to_range_test() {
        assert_eq!(convert_section_assignment_to_range("1-3"), 1..=3);
        assert_eq!(convert_section_assignment_to_range("6-6"), 6..=6);
        assert_eq!(convert_section_assignment_to_range("2-7"), 2..=7);
        assert_eq!(convert_section_assignment_to_range("100-1000"), 100..=1000);
        assert_eq!(convert_section_assignment_to_range("12-13"), 12..=13);
    }

    #[test]
    pub fn parse_selection_ranges_test() {
        // single
        assert_eq!(parse_section_ranges("1-2,3-4"), vec![(1..=2, 3..=4)]);

        // multiple
        assert_eq!(
            parse_section_ranges("1-2,3-4\n5-6,7-8\n9-10,11-12"),
            vec![(1..=2, 3..=4), (5..=6, 7..=8), (9..=10, 11..=12)]
        );
    }

    #[test]
    pub fn part_one_camp_cleanup_test() {
        assert_eq!(part_one::camp_cleanup("1-2,3-4"), 0);
        assert_eq!(part_one::camp_cleanup("1-2,1-1"), 1);
        assert_eq!(part_one::camp_cleanup("1-2,1-1\n3-4,4-6\n7-9,7-8"), 2);
    }

    #[test]
    pub fn part_two_camp_cleanup_test() {
        assert_eq!(part_two::camp_cleanup("1-2,3-4"), 0);
        assert_eq!(part_two::camp_cleanup("1-2,1-1"), 1);
        assert_eq!(part_two::camp_cleanup("1-2,1-3\n3-10,4-12\n7-9,7-12"), 3);
    }
}
