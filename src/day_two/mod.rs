#[allow(dead_code)]
pub mod problem {
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Choice {
        Rock,
        Paper,
        Scissors,
    }

    impl FromStr for Choice {
        type Err = ();

        /// Converts an input to an enum
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match input {
                "A" | "X" => Ok(Choice::Rock),
                "B" | "Y" => Ok(Choice::Paper),
                "C" | "Z" => Ok(Choice::Scissors),
                _ => Err(()),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Outcome {
        Win,
        Loss,
        Draw,
    }

    impl FromStr for Outcome {
        type Err = ();

        /// Converts an input to an enum
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match input {
                "X" => Ok(Outcome::Loss),
                "Y" => Ok(Outcome::Draw),
                "Z" => Ok(Outcome::Win),
                _ => Err(()),
            }
        }
    }

    /// Converts a raw input of new line separated tuples into a vector
    /// the tuples represent the choices you are supposed to make based on the strategy guide
    pub fn convert_input_to_rounds(input: &str) -> Vec<(&str, &str)> {
        input
            .split("\n")
            .map(|row| {
                let mut choices = row.split(" ");
                let tuple = (choices.next().unwrap(), choices.next().unwrap());

                tuple
            })
            .collect::<Vec<(&str, &str)>>()
    }

    /// Calculates the score based on the outcome of the match and the choice you made
    /// A/X = Rock (1 point), B/Y = Paper (2 points), C/Z = Scissors (3 points)
    /// Win = 6, Draw = 3, Loss = 0
    /// NOTE: score are relative to player A
    pub fn calculate_score(a: Choice, b: Choice) -> u32 {
        let outcome_value = calculate_outcome_score(a, b);
        let choice_value = calculate_pick_score(a);

        outcome_value + choice_value
    }

    /// Calculates the score based on the outcome of the match
    /// Win = 6, Draw = 3, Loss = 0
    pub fn calculate_outcome_score(a: Choice, b: Choice) -> u32 {
        match a {
            Choice::Rock => match b {
                Choice::Rock => 3,
                Choice::Paper => 0,
                Choice::Scissors => 6,
            },
            Choice::Paper => match b {
                Choice::Rock => 6,
                Choice::Paper => 3,
                Choice::Scissors => 0,
            },
            Choice::Scissors => match b {
                Choice::Rock => 0,
                Choice::Paper => 6,
                Choice::Scissors => 3,
            },
        }
    }

    /// Calculates the score based on the pick
    /// Rock = 1, Paper = 2, Scissors = 3
    pub fn calculate_pick_score(a: Choice) -> u32 {
        match a {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub mod part_one {
        use std::str::FromStr;

        use super::{calculate_score, convert_input_to_rounds, Choice};

        /// Calculates the total score based on the encrypted strategy guide given from an elf
        pub fn rock_paper_scissors(input: &str) -> u32 {
            let mut total_score = 0;

            let rounds = convert_input_to_rounds(input);

            for round in rounds {
                let (opponent, me) = round;

                let round_score = calculate_score(
                    Choice::from_str(me).unwrap(),
                    Choice::from_str(opponent).unwrap(),
                );

                total_score = round_score + total_score;
            }

            total_score
        }
    }

    pub mod part_two {
        use std::str::FromStr;

        use super::{calculate_score, convert_input_to_rounds, Choice, Outcome};

        pub fn rock_paper_scissors(input: &str) -> u32 {
            let mut total_score = 0;

            let rounds = convert_input_to_rounds(input);

            for round in rounds {
                let (opponent, desired_outcome) = round;

                let your_pick = determine_pick_to_achieve_desired_outcome(
                    Outcome::from_str(desired_outcome).unwrap(),
                    Choice::from_str(opponent).unwrap(),
                );

                let round_score = calculate_score(your_pick, Choice::from_str(opponent).unwrap());

                total_score = round_score + total_score;
            }

            total_score
        }

        /// Determines what you should pick to achieve the desired outcome
        /// X = Loss, Y = Draw, Z = Win
        pub fn determine_pick_to_achieve_desired_outcome(
            desired_outcome: Outcome,
            opponent_pick: Choice,
        ) -> Choice {
            match desired_outcome {
                Outcome::Win => match opponent_pick {
                    Choice::Rock => Choice::Paper,
                    Choice::Paper => Choice::Scissors,
                    Choice::Scissors => Choice::Rock,
                },
                Outcome::Loss => match opponent_pick {
                    Choice::Rock => Choice::Scissors,
                    Choice::Paper => Choice::Rock,
                    Choice::Scissors => Choice::Paper,
                },
                Outcome::Draw => match opponent_pick {
                    Choice::Rock => Choice::Rock,
                    Choice::Paper => Choice::Paper,
                    Choice::Scissors => Choice::Scissors,
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::problem::{
        calculate_outcome_score, calculate_pick_score, part_one, part_two, Choice, Outcome,
    };
    use std::str::FromStr;

    #[test]
    fn choice_from_str() {
        assert_eq!(Choice::from_str("A").unwrap(), Choice::Rock);
        assert_eq!(Choice::from_str("X").unwrap(), Choice::Rock);
        assert_eq!(Choice::from_str("B").unwrap(), Choice::Paper);
        assert_eq!(Choice::from_str("Y").unwrap(), Choice::Paper);
        assert_eq!(Choice::from_str("C").unwrap(), Choice::Scissors);
        assert_eq!(Choice::from_str("Z").unwrap(), Choice::Scissors);
    }

    #[test]
    fn outcome_from_str() {
        assert_eq!(Outcome::from_str("X").unwrap(), Outcome::Loss);
        assert_eq!(Outcome::from_str("Y").unwrap(), Outcome::Draw);
        assert_eq!(Outcome::from_str("Z").unwrap(), Outcome::Win);
    }

    #[test]
    fn calculate_pick_score_test() {
        assert_eq!(calculate_pick_score(Choice::Rock), 1);
        assert_eq!(calculate_pick_score(Choice::Paper), 2);
        assert_eq!(calculate_pick_score(Choice::Scissors), 3);
    }

    #[test]
    fn calculate_outcome_score_test() {
        // Wins
        assert_eq!(calculate_outcome_score(Choice::Rock, Choice::Scissors), 6);
        assert_eq!(calculate_outcome_score(Choice::Paper, Choice::Rock), 6);
        assert_eq!(calculate_outcome_score(Choice::Scissors, Choice::Paper), 6);

        // Losses
        assert_eq!(calculate_outcome_score(Choice::Rock, Choice::Paper), 0);
        assert_eq!(calculate_outcome_score(Choice::Paper, Choice::Scissors), 0);
        assert_eq!(calculate_outcome_score(Choice::Scissors, Choice::Rock), 0);

        // Draws
        assert_eq!(calculate_outcome_score(Choice::Rock, Choice::Rock), 3);
        assert_eq!(calculate_outcome_score(Choice::Paper, Choice::Paper), 3);
        assert_eq!(
            calculate_outcome_score(Choice::Scissors, Choice::Scissors),
            3
        );
    }

    #[test]
    pub fn part_one_rock_paper_scissors_test() {
        assert_eq!(part_one::rock_paper_scissors("A X\nC Y\nC Y"), 8);
        assert_eq!(part_one::rock_paper_scissors("B Y\nA Z\nB X"), 9);
        assert_eq!(part_one::rock_paper_scissors("C Z\nB X\nA Z"), 10);
    }

    #[test]
    pub fn part_two_rock_paper_scissors_test() {
        assert_eq!(part_two::rock_paper_scissors("A X\nC Y\nC Y"), 15);
        assert_eq!(part_two::rock_paper_scissors("B Y\nA Z\nB X"), 14);
        assert_eq!(part_two::rock_paper_scissors("C Z\nB X\nA Z"), 16);
    }
}
