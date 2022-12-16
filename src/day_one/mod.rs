#[allow(dead_code)]
pub mod problem {
    /// Converts "meal log" (new-line separated string of numbers) into a single u32 value that represents the sum of the meals
    /// example: input = "1\n\2\n3" -> 6
    pub fn calculate_sum_of_meals(input: &str) -> u32 {
        let meals: Vec<u32> = input
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&v| str::parse::<u32>(&v).unwrap())
            .collect();

        meals.iter().sum()
    }

    pub mod part_one {
        use super::calculate_sum_of_meals;

        /// Counts calories and returns the number of calories eaten by the elf who consumed the most
        pub fn calorie_counting(input: &str) -> u32 {
            let elf_meals = input.split("\n\n").collect::<Vec<&str>>();

            let mut calories_consumed: Vec<u32> = Vec::new();

            for meal in elf_meals {
                let calories = calculate_sum_of_meals(meal);

                calories_consumed.push(calories);
            }

            let max = calories_consumed.iter().max().unwrap().to_owned();

            max
        }
    }

    pub mod part_two {
        use super::calculate_sum_of_meals;

        /// Converts "meal log" (new-line separated string of numbers) into a single u32 value that represents the
        /// sum of calories consumed by the top three elves
        /// example: input = "1\n\2\n3" -> 6
        pub fn calorie_counting(input: &str) -> u32 {
            let elf_meals = input.split("\n\n").collect::<Vec<&str>>();

            let mut calories_consumed: Vec<u32> = Vec::new();

            for meal in elf_meals {
                let calories = calculate_sum_of_meals(meal);

                calories_consumed.push(calories);
            }

            calories_consumed.sort();

            calories_consumed[calories_consumed.len() - 3..]
                .iter()
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::problem::{calculate_sum_of_meals, part_one, part_two};

    #[test]
    fn calculate_sum_of_meals_test() {
        assert_eq!(calculate_sum_of_meals("1\n2\n3"), 6);
        assert_eq!(calculate_sum_of_meals("2\n2\n2"), 6);
        assert_eq!(calculate_sum_of_meals("5\n10\n15\n20"), 50);
    }

    #[test]
    fn part_one_calorie_counting_test() {
        assert_eq!(part_one::calorie_counting("1\n2\n3"), 6);
    }

    #[test]
    fn part_two_calorie_counting_test() {
        assert_eq!(part_two::calorie_counting("5\n\n5\n\n5"), 15);
    }
}
