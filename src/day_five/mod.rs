pub mod problem {
    #[derive(Debug)]
    pub struct Instruction {
        count: u8,
        destination: u8,
        origin: u8,
    }

    #[derive(Debug)]
    pub struct Stack {
        elements: Vec<char>,
    }

    impl Stack {
        pub fn new(elements: Vec<char>) -> Self {
            Stack { elements }
        }

        pub fn pop(&mut self) -> char {
            self.elements.remove(self.elements.len() - 1)
        }

        pub fn push(&mut self, char: char) -> () {
            self.elements.push(char)
        }

        pub fn peek(&self) -> char {
            self.elements[self.elements.len() - 1]
        }
    }

    pub mod part_one {
        use regex::Regex;

        use super::{Instruction, Stack};

        /// Calculates the top of each stack after an intial set of stacks has been shuffled around by some elves as per the instructions left by santa
        pub fn supply_stacks(instructions: &str, stacks: &str) -> String {
            let mut stacks = parse_stacks(stacks)
                .iter()
                .map(|s| Stack::new(s.to_vec()))
                .collect::<Vec<Stack>>();

            let instructions = parse_instructions(instructions);

            for instruction in instructions {
                let mut operations = 0;
                let mut elements: Vec<char> = Vec::new();

                while operations < instruction.count {
                    elements.push(stacks[usize::from(instruction.origin - 1)].pop());

                    operations = operations + 1;
                }

                for element in elements {
                    stacks[usize::from(instruction.destination - 1)].push(element)
                }
            }

            stacks
                .iter()
                .map(|s| s.peek().to_string())
                .collect::<Vec<String>>()
                .join("")
        }

        pub fn parse_stacks(input: &str) -> Vec<Vec<char>> {
            input.split("\n").map(|s| s.chars().collect()).collect()
        }

        /// Parses a vector of instructions from a new line separated list
        /// example: "move 1 from 5 to 6\nmove 2 from 1 to 4" -> [Instruction {...}, Instruction {...}]
        pub fn parse_instructions(input: &str) -> Vec<Instruction> {
            input
                .split("\n")
                .map(|l| convert_str_to_instruction(l))
                .collect::<Vec<Instruction>>()
        }

        /// Converts an elf's instructions to a struct
        pub fn convert_str_to_instruction(str: &str) -> Instruction {
            let re = Regex::new("\\d+").unwrap();

            let mut matches = re.find_iter(str);

            // This is the same as a 3x left shift on the matches iter.
            // TODO: convert to shift for terseness
            let count = matches.nth(0).unwrap().as_str().parse::<u8>().unwrap();
            let origin = matches.nth(0).unwrap().as_str().parse::<u8>().unwrap();
            let destination = matches.nth(0).unwrap().as_str().parse::<u8>().unwrap();

            Instruction {
                count,
                destination,
                origin,
            }
        }
    }
}
