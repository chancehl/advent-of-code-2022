pub mod problem {
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl FromStr for Direction {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "U" => Ok(Direction::Up),
                "D" => Ok(Direction::Down),
                "L" => Ok(Direction::Left),
                "R" => Ok(Direction::Right),
                _ => Err(()),
            }
        }
    }

    #[derive(Debug)]
    pub struct Instruction {
        direction: Direction,
        count: u8,
    }

    impl FromStr for Instruction {
        type Err = ();

        /// Converts a string to an instruction object
        /// example: "L 4" => Instruction { direction: Left, count: 4 }
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(" ");

            let direction = Direction::from_str(
                parts
                    .nth(0)
                    .expect("Could not locate direction in raw instruction"),
            )
            .unwrap();

            let count = parts
                .nth(0)
                .expect("Could not locate count from raw instruction")
                .parse::<u8>()
                .expect("Could not parse u8 from count in raw instruction");

            Ok(Instruction { direction, count })
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Token {
        Head,
        Tail,
        Both,
        Empty,
    }

    impl Token {
        fn to_char(&self) -> char {
            match &self {
                Token::Both => 'B',
                Token::Head => 'H',
                Token::Tail => 'T',
                Token::Empty => '.',
            }
        }
    }

    type Node = Option<Token>;

    type Coords = (usize, usize);

    #[derive(Debug)]
    pub struct ExpandingGraph {
        /// Vec of nodes
        elements: Vec<Vec<Node>>,
        /// This tracks everywhere the tail has been
        tail_posns: Vec<Coords>,
    }

    impl ExpandingGraph {
        /// Creates a new instance of an expanding graph
        pub fn new(starting_posn: Coords) -> Self {
            let (x, y) = starting_posn;

            let mut elements = Vec::new();

            for i in 0..x.max(1) {
                let mut cols: Vec<Node> = Vec::new();

                for j in 0..y.max(1) {
                    if i == x && j == y {
                        // both elements always start in the same position
                        cols.push(Some(Token::Both));
                    } else {
                        cols.push(None);
                    }
                }

                elements.push(cols);
            }

            ExpandingGraph {
                elements,
                tail_posns: vec![starting_posn],
            }
        }

        /// Adds a row to an ExpandingGraph
        fn add_row(&mut self) -> () {
            let columns = self.elements[0].len();

            let mut new_row = Vec::new();

            for _ in 0..columns {
                new_row.push(None);
            }

            self.elements.push(new_row)
        }

        /// Adds a column to an expanding graph
        fn add_column(&mut self) -> () {
            let mut new_elements: Vec<Vec<Node>> = Vec::new();

            for row in &self.elements {
                let mut new_row = row.clone();

                new_row.push(None);

                new_elements.push(new_row);
            }

            self.elements = new_elements
        }

        /// Increases the size of the matrix by n rows and columns
        fn grow(&mut self, n: usize) -> () {
            let mut i = 0;

            while i < n {
                self.add_column();
                self.add_row();

                i = i + 1;
            }
        }

        /// Moves the head based off of a given intruction
        pub fn move_pointer(&mut self, instruction: Instruction) -> () {
            match instruction.direction {
                Direction::Up => self.move_up(instruction.count.into()),
                Direction::Down => self.move_down(instruction.count.into()),
                Direction::Left => self.move_left(instruction.count.into()),
                Direction::Right => self.move_right(instruction.count.into()),
            }
        }

        /// Moves the head (and tail if applicable) up n spaces
        fn move_up(&mut self, n: usize) -> () {
            todo!()
        }

        /// Moves the head (and tail if applicable) down n spaces
        fn move_down(&mut self, n: usize) -> () {
            todo!()
        }

        /// Moves the head (and tail if applicable) left n spaces
        fn move_left(&mut self, n: usize) -> () {
            todo!()
        }

        /// Moves the head (and tail if applicable) right n spaces
        fn move_right(&mut self, n: usize) -> () {
            // Do the moves one at a time
            let mut moves_taken = 0;

            while moves_taken < n {
                let (head_row, head_col) = self
                    .get_posn(Token::Head)
                    .expect("Could not find head position in graph");

                let (tail_row, tail_col) = self
                    .get_posn(Token::Tail)
                    .expect("Could not locate tail posn");

                if head_col + 1 <= n {
                    self.grow(1);
                }

                // Move the head one to the right
                self.update_posn(Token::Head, (head_row, head_col + 1));

                let is_tail_adjancent = self.is_tail_adjacent_to_head();

                if !is_tail_adjancent {
                    self.update_posn(Token::Tail, (tail_row, tail_col + 1));
                }

                moves_taken = moves_taken + 1;

                println!("Move {:?}", moves_taken);
                println!("{}", self.to_string());
            }
        }

        /// Updates the position of a token (note: this will mutate self.elements and self.tail_posns)
        fn update_posn(&mut self, token: Token, new_posn: Coords) -> () {
            let (new_row, new_col) = new_posn;

            let (head_row, head_col) = self
                .get_posn(Token::Head)
                .expect("Could not locate head position");

            let (tail_row, tail_col) = self
                .get_posn(Token::Tail)
                .expect("Could not locate tail posn");

            if token == Token::Head {
                // Move token to the new position
                if new_row == tail_row && new_col == tail_col {
                    self.elements[new_row][new_col] = Some(Token::Both);
                } else {
                    self.elements[new_row][new_col] = Some(Token::Head);
                }

                // Update old position
                if self.elements[head_row][head_col] == Some(Token::Both) {
                    self.elements[head_row][head_col] = Some(Token::Tail);
                } else {
                    self.elements[head_row][head_col] = None;
                }
            } else if token == Token::Tail {
                // Move token to the new position
                if new_row == head_row && new_col == head_col {
                    self.elements[new_row][new_col] = Some(Token::Both);
                } else {
                    self.elements[new_row][new_col] = Some(Token::Tail);
                }

                // Update old position
                if self.elements[tail_row][tail_col] == Some(Token::Both) {
                    self.elements[tail_row][tail_col] = Some(Token::Head);
                } else {
                    self.elements[tail_row][tail_col] = None;
                }

                // Special logic for tail updates: always store the new position
                self.tail_posns.push(new_posn);
            }
        }

        /// Gets the current position of the specified token
        fn get_posn(&self, token: Token) -> Option<Coords> {
            for i in 0..self.elements[0].len() {
                for j in 0..self.elements.len() {
                    if self.elements[i][j] == Some(Token::Both)
                        || self.elements[i][j] == Some(token)
                    {
                        return Some((i, j));
                    }
                }
            }

            None
        }

        /// Determines if the tail is adjacent to the head
        fn is_tail_adjacent_to_head(&self) -> bool {
            let (head_row, head_col) = self.get_posn(Token::Head).expect("Could not locate head");
            let (tail_row, tail_col) = self.get_posn(Token::Tail).expect("Could not locate tail");

            (head_row - tail_row) <= 1 && (head_col - tail_col) <= 1
        }
    }

    impl ToString for ExpandingGraph {
        /// Converts an expanding graph to string format
        fn to_string(&self) -> String {
            let mut output = "".to_owned();

            for row in &self.elements {
                for data in row {
                    if let Some(d) = data {
                        output.push(d.to_char())
                    } else {
                        output.push('.')
                    }
                }

                output.push('\n')
            }

            output
        }
    }

    /// Converts a new line separated input to a vec of instructions
    pub fn convert_input_to_instructions(input: &str) -> Vec<Instruction> {
        input
            .split("\n")
            .map(|l| {
                Instruction::from_str(l).expect("Could not convert line to instruction struct")
            })
            .collect::<Vec<Instruction>>()
    }

    pub mod part_one {
        use super::{convert_input_to_instructions, ExpandingGraph};

        pub fn rope_bridge(input: &str) -> usize {
            let instructions = convert_input_to_instructions(input);

            let mut graph = ExpandingGraph::new((0, 0));

            for instruction in instructions {
                println!("processing instruction = {:?}", instruction);

                graph.move_pointer(instruction);
            }

            graph.tail_posns.len()
        }
    }
}
