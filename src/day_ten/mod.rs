pub mod problem {
    use std::{collections::VecDeque, str::FromStr};

    #[derive(Debug, PartialEq)]
    enum Instruction {
        NOOP,
        ADDX,
    }

    impl FromStr for Instruction {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "noop" => Ok(Instruction::NOOP),
                "addx" => Ok(Instruction::ADDX),
                _ => Err(()),
            }
        }
    }

    #[derive(Debug)]
    struct Operation {
        instruction: Instruction,
        data: Option<i32>,
    }

    impl FromStr for Operation {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(" ");

            let instruction_raw = parts.nth(0).expect("Could not locate instruction");

            let instruction = Instruction::from_str(instruction_raw)
                .expect("Could not parse instruction from string");

            if let Some(data) = parts.nth(0) {
                return Ok(Operation {
                    instruction,
                    data: Some(data.parse::<i32>().expect("")),
                });
            }

            return Ok(Operation {
                instruction,
                data: None,
            });
        }
    }

    struct SimpleCPU {
        breakpoints: VecDeque<i32>,
        operations: Vec<Operation>,
    }

    impl SimpleCPU {
        /// Creates a new instance of a simple elf CPU
        pub fn new(operations: Vec<Operation>, breakpoints: VecDeque<i32>) -> Self {
            SimpleCPU {
                operations,
                breakpoints,
            }
        }

        pub fn execute(&mut self) -> i32 {
            let mut cycle = 0;
            let mut register = 1;
            let mut signal_strength = 0;

            for operation in &self.operations {
                if operation.instruction == Instruction::NOOP {
                    cycle = cycle + 1;
                } else {
                    let data = operation.data.unwrap_or(0);

                    cycle = cycle + 2;
                    register = register + data;
                }

                if let Some(breakpoint) = self.breakpoints.get(0) {
                    // First check to see if we're exactly equal to the breakpoint
                    // otherwise check to see if we're within 2 since at most we will be off by 1/2 if we aren't exactly equal
                    if cycle == *breakpoint || (breakpoint - cycle) <= 2 {
                        // update signal strength
                        signal_strength = signal_strength + (breakpoint * register);

                        // remove breakpoint
                        self.breakpoints.pop_front();
                    }
                } else {
                    return signal_strength;
                }
            }

            signal_strength
        }
    }

    pub mod part_one {
        use std::{collections::VecDeque, str::FromStr};

        use super::{Operation, SimpleCPU};

        pub fn cathode_ray_tube(input: &str) -> i32 {
            let operations = input
                .split("\n")
                .map(|l| Operation::from_str(l).expect("Could not convert line to operation"))
                .collect::<Vec<Operation>>();

            let breakpoints = VecDeque::from_iter([20, 60, 100, 140, 180, 220]);

            SimpleCPU::new(operations, breakpoints).execute()
        }
    }
}
