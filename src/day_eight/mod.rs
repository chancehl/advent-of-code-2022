pub mod problem {
    use std::str::FromStr;

    pub struct ElfTreeGraph<T> {
        pub trees: Vec<Vec<T>>,
    }

    type AdjacentNodes = (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>);

    impl ElfTreeGraph<u32> {
        /// Creates a new elf graph object
        pub fn new(elements: Vec<Vec<u32>>) -> Self {
            ElfTreeGraph { trees: elements }
        }

        /// Get all trees north of a node
        pub fn get_north_trees(&self, row: usize, col: usize) -> Vec<u32> {
            let mut elements: Vec<u32> = Vec::new();

            for i in 0..row {
                elements.push(self.trees[i][col]);
            }

            elements
        }

        /// Gets all trees south of a node
        pub fn get_south_trees(&self, row: usize, col: usize) -> Vec<u32> {
            let mut elements: Vec<u32> = Vec::new();

            for i in (row + 1)..self.trees.len() {
                elements.push(self.trees[i][col]);
            }

            elements
        }

        /// Gets all trees east of a node
        pub fn get_east_trees(&self, row: usize, col: usize) -> Vec<u32> {
            let mut elements: Vec<u32> = Vec::new();

            for i in (col + 1)..self.trees[row].len() {
                elements.push(self.trees[row][i])
            }

            elements
        }

        /// Gets all trees west of a node
        pub fn get_west_trees(&self, row: usize, col: usize) -> Vec<u32> {
            let mut elements: Vec<u32> = Vec::new();

            for i in 0..col {
                elements.push(self.trees[row][i])
            }

            elements
        }

        /// Gets all adjacent trees for a node
        pub fn get_adjacent_trees(&self, row: usize, col: usize) -> AdjacentNodes {
            (
                self.get_north_trees(row, col),
                self.get_south_trees(row, col),
                self.get_east_trees(row, col),
                self.get_west_trees(row, col),
            )
        }

        /// Counts the edges of the elf tree graph
        pub fn count_edges(&self) -> usize {
            let mut count = 0;

            let rows = self.trees.len();
            let cols = self.trees[0].len();

            for _ in 1..rows - 1 {
                for _ in 1..cols - 1 {
                    count = count + 1;
                }
            }

            (rows * cols) - count
        }

        /// Determines whether or not a tree is visible from a given direction
        pub fn is_visible_from_direction(
            &self,
            row: usize,
            col: usize,
            adjacent_trees: Vec<u32>,
        ) -> bool {
            let tree = self.trees[row][col];

            !adjacent_trees.iter().any(|v| v >= &tree)
        }

        /// Determines whether or not a tree is visible from any direction
        pub fn is_tree_visible(&self, row: usize, col: usize) -> bool {
            let (north, south, east, west) = self.get_adjacent_trees(row, col);

            self.is_visible_from_direction(row, col, north)
                || self.is_visible_from_direction(row, col, south)
                || self.is_visible_from_direction(row, col, east)
                || self.is_visible_from_direction(row, col, west)
        }

        /// Calculates the number of trees between a tree (graph[row][col]) that are strictly less than the size of said tree
        pub fn calculcate_directional_visibility(
            &self,
            row: usize,
            col: usize,
            adjacent_trees: Vec<u32>,
        ) -> u32 {
            let tree = self.trees[row][col];

            if adjacent_trees.len() == 1 {
                return 1;
            } else {
                // we start at 1 because we always assume the tree immediately next to this tree is visible
                let mut visible_trees = 0;

                let mut index = 0;

                while index < adjacent_trees.len() {
                    let tree_comparison = adjacent_trees[index];

                    if tree_comparison < tree {
                        visible_trees = visible_trees + 1;
                    } else if tree_comparison == tree {
                        visible_trees = visible_trees + 1;
                        return visible_trees;
                    } else {
                        visible_trees = visible_trees + 1;
                        return visible_trees;
                    }

                    index = index + 1;
                }

                return visible_trees;
            }
        }

        /// Calculates a trees "scenic score"
        /// A scenic score is the number of trees between the tree at graph[row][col] and any given edge which is taller than or the same size of graph[row][col]
        pub fn calculate_scenic_score(&self, row: usize, col: usize) -> u32 {
            let (mut north, south, east, mut west) = self.get_adjacent_trees(row, col);

            // Reverse these because from the tree's perspective they are in the opposite order
            north.reverse();
            west.reverse();

            self.calculcate_directional_visibility(row, col, north)
                * self.calculcate_directional_visibility(row, col, south)
                * self.calculcate_directional_visibility(row, col, east)
                * self.calculcate_directional_visibility(row, col, west)
        }
    }

    impl FromStr for ElfTreeGraph<u32> {
        type Err = ();

        /// Converts an input to a ElfTreeGraph object
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let rows = s.split("\n").collect::<Vec<&str>>();
            let elements = rows
                .iter()
                .map(|row| {
                    row.chars()
                        .map(|c| {
                            c.to_string()
                                .parse::<u32>()
                                .expect("Could not convert char to u32")
                        })
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();

            Ok(ElfTreeGraph::new(elements))
        }
    }

    pub mod part_one {
        use std::str::FromStr;

        use super::ElfTreeGraph;

        /// Counts the trees that are visible from the edge of the forest based on an elf tree graph.
        /// A tree is visible when it is the tallest tree in your sight line (meaning no other trees between you and it are larger than it)
        pub fn treetop_tree_house(input: &str) -> usize {
            let graph =
                ElfTreeGraph::from_str(input).expect("Could not convert input to Graph type");

            let rows = graph.trees.len();
            let cols = graph.trees[0].len();

            let mut visible_trees = 0;

            // every edge counts because there's nothing blocking the view
            let edges = graph.count_edges();

            for i in 1..rows - 1 {
                for j in 1..cols - 1 {
                    let visibility = graph.is_tree_visible(i, j);

                    if visibility {
                        visible_trees = visible_trees + 1;
                    }
                }
            }

            edges + visible_trees
        }
    }

    pub mod part_two {
        use std::str::FromStr;

        use super::ElfTreeGraph;

        pub fn treetop_tree_house(input: &str) -> u32 {
            let graph = ElfTreeGraph::from_str(input)
                .expect("Could not convert input to ElfTreeGraph type");

            let rows = graph.trees.len();
            let cols = graph.trees[0].len();

            let mut scenic_scores: Vec<u32> = Vec::new();

            for i in 1..rows - 1 {
                for j in 1..cols - 1 {
                    let scenic_score = graph.calculate_scenic_score(i, j);

                    scenic_scores.push(scenic_score);
                }
            }

            scenic_scores
                .iter()
                .max()
                .expect("Could not find max for scenic_score vec")
                .to_owned()
        }
    }
}
