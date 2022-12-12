#[allow(dead_code)]
pub mod problem {
    pub mod part_one {
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

        /// Converts "meal log" (new-line separated string of numbers) into a single u32 value that represents the sum of the meals
        /// Example: input = "1\n\2\n3" -> 6
        pub fn calculate_sum_of_meals(input: &str) -> u32 {
            let meals: Vec<u32> = input
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&v| str::parse::<u32>(&v).unwrap())
                .collect();

            meals.iter().sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::problem::part_one::{calculate_sum_of_meals, calorie_counting};

    #[test]
    fn calculate_sum_of_meals_test() {
        let input = "1\n2\n3";
        let result = calculate_sum_of_meals(input);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn part_one_calorie_counting_test() {
        let input = "1\n2\n3\n\n5\n5\n5";
        let result = calorie_counting(input);
        let expected = 15;
        assert_eq!(result, expected);
    }
}
