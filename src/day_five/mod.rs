#[allow(dead_code)]
pub mod problem {
    use regex::Regex;

    #[derive(Debug, PartialEq)]
    pub struct Instruction {
        pub count: u8,
        pub destination: u8,
        pub origin: u8,
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

    /// Parses an input into stack structs
    pub fn parse_shipping_containers(input: &str) -> Vec<Vec<char>> {
        input.split("\n").map(|s| s.chars().collect()).collect()
    }

    pub mod part_one {
        use super::{parse_instructions, parse_shipping_containers, Stack};

        /// Calculates the top of each stack after an intial set of stacks has been shuffled around by some elves as per the instructions left by santa
        pub fn supply_stacks(instructions: &str, stacks: &str) -> String {
            let mut shipping_containers = parse_shipping_containers(stacks)
                .iter()
                .map(|s| Stack::new(s.to_vec()))
                .collect::<Vec<Stack>>();

            let instructions = parse_instructions(instructions);

            for instruction in instructions {
                let mut operations = 0;
                let mut elements: Vec<char> = Vec::new();

                while operations < instruction.count {
                    elements.push(shipping_containers[usize::from(instruction.origin - 1)].pop());

                    operations = operations + 1;
                }

                for element in elements {
                    shipping_containers[usize::from(instruction.destination - 1)].push(element)
                }
            }

            shipping_containers
                .iter()
                .map(|s| s.peek().to_string())
                .collect::<Vec<String>>()
                .join("")
        }
    }

    pub mod part_two {
        use super::{parse_instructions, parse_shipping_containers, Stack};

        /// Calculates the top of each stack after an intial set of stacks has been shuffled around by some elves as per the instructions left by santa
        pub fn supply_stacks(instructions: &str, containers: &str) -> String {
            let mut shipping_containers = parse_shipping_containers(containers)
                .iter()
                .map(|s| Stack::new(s.to_vec()))
                .collect::<Vec<Stack>>();

            let instructions = parse_instructions(instructions);

            for instruction in instructions {
                let mut operations = 0;
                let mut elements: Vec<char> = Vec::new();

                while operations < instruction.count {
                    elements.push(shipping_containers[usize::from(instruction.origin - 1)].pop());

                    operations = operations + 1;
                }

                // Reverse the elements because now we can select multiple at once and they come out in LIFO order
                // but need to be reapplied in reverse
                elements.reverse();

                for element in elements {
                    shipping_containers[usize::from(instruction.destination - 1)].push(element)
                }
            }

            shipping_containers
                .iter()
                .map(|s| s.peek().to_string())
                .collect::<Vec<String>>()
                .join("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::problem::{convert_str_to_instruction, part_one, part_two, Instruction};

    #[test]
    pub fn convert_str_to_instruction_test() {
        assert_eq!(
            convert_str_to_instruction("move 1 from 5 to 3"),
            Instruction {
                count: 1,
                destination: 3,
                origin: 5
            }
        );
        assert_eq!(
            convert_str_to_instruction("move 2 from 3 to 1"),
            Instruction {
                count: 2,
                destination: 1,
                origin: 3
            }
        );
        assert_eq!(
            convert_str_to_instruction("move 10 from 5 to 20"),
            Instruction {
                count: 10,
                destination: 20,
                origin: 5
            }
        );
    }

    #[test]
    pub fn part_one_supply_stacks_test() {
        // TODO: write more of these to validate
        assert_eq!(
            part_one::supply_stacks("move 1 from 2 to 1\nmove 1 from 3 to 2", "AAA\nBBB\nCCC"),
            "BCC"
        );
    }

    #[test]
    pub fn part_two_supply_stacks_test() {
        // TODO: write more of these to validate
        assert_eq!(
            part_two::supply_stacks("move 2 from 2 to 1\nmove 1 from 3 to 2", "ABC\nHIJ\nXYZ"),
            "JZY"
        );
    }
}
