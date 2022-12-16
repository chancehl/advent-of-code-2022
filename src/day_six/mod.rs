#[allow(dead_code)]
pub mod problem {
    use std::collections::HashMap;

    /// Calibrates an elf tuning device by looking for looking for the first group of N distinct chars
    /// and returning the index of the last character
    /// example: input = "bvwbjpl", distinct chars = 4 -> index 5
    /// note: the problem calls for 1-based indexing
    pub fn tuning_trouble(input: &str, distinct_chars: usize) -> Option<usize> {
        let mut window_start = 0;
        let mut window_end = window_start + (distinct_chars - 1);

        while window_end < input.len() {
            let slice = &input[window_start..=window_end];

            if !has_duplicate_chars(slice) {
                return Some(window_end + 1);
            }

            window_start = window_start + 1;
            window_end = window_end + 1;
        }

        return None;
    }

    /// Determines if a string contains duplicate characters
    pub fn has_duplicate_chars(str: &str) -> bool {
        let mut map: HashMap<char, usize> = HashMap::new();

        for char in str.chars() {
            if map.contains_key(&char) {
                return true;
            } else {
                map.insert(char, 0);
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::problem::{has_duplicate_chars, tuning_trouble};

    #[test]
    fn has_duplicate_chars_test() {
        assert_eq!(has_duplicate_chars("str"), false);
        assert_eq!(has_duplicate_chars("aabbcc"), true);
        assert_eq!(has_duplicate_chars("chance"), true);
        assert_eq!(has_duplicate_chars("christmas"), true);
    }

    #[test]
    fn tuning_trouble_test() {
        assert_eq!(
            tuning_trouble("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(),
            5
        );
        assert_eq!(
            tuning_trouble("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(),
            6
        );
        assert_eq!(
            tuning_trouble("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
            10
        );
        assert_eq!(
            tuning_trouble("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(),
            11
        );

        assert_eq!(
            tuning_trouble("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
        assert_eq!(
            tuning_trouble("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(),
            23
        );
        assert_eq!(
            tuning_trouble("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(),
            29
        );
        assert_eq!(
            tuning_trouble("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(),
            26
        );
    }
}
