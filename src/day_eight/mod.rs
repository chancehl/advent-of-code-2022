pub mod problem {
    use std::str::FromStr;

    pub struct ElfTreeGraph<T> {
        pub trees: Vec<Vec<T>>,
    }

    type DirectionalVisibility = (bool, bool, bool, bool);

    type AdjacentNodes = (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>);

    impl ElfTreeGraph<u32> {
        pub fn new(elements: Vec<Vec<u32>>) -> Self {
            ElfTreeGraph { trees: elements }
        }

        pub fn get_adjacent_trees(&self, row: usize, col: usize) -> AdjacentNodes {
            todo!()
        }

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

        pub fn is_tree_visible(&self, row: usize, col: usize) -> DirectionalVisibility {
            let tree = self.trees[row][col];
            let adjacent_nodes = self.get_adjacent_trees(row, col);

            (false, false, false, false)
        }
    }

    impl FromStr for ElfTreeGraph<u32> {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(ElfTreeGraph::new(
                s.split("\n")
                    .map(|row| {
                        row.chars()
                            .map(|c| {
                                c.to_string()
                                    .parse::<u32>()
                                    .expect("Could not parse u32 from char")
                            })
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>(),
            ))
        }
    }

    pub mod part_one {
        use std::str::FromStr;

        use super::ElfTreeGraph;

        pub fn treetop_tree_house(input: &str) -> usize {
            let graph =
                ElfTreeGraph::from_str(input).expect("Could not convert input to Graph type");

            let rows = graph.trees.len();
            let cols = graph.trees[0].len();

            let mut visible_trees = 0;

            // every edge counts because there's nothing blocking the view
            let edges = graph.count_edges();

            println!("edges = {:?}", edges);

            for i in 1..rows - 1 {
                for j in 1..cols - 1 {
                    println!("graph[i][j] = {:?}", graph.trees[i][j]);

                    let visibility = graph.is_tree_visible(i, j);

                    println!("visibility = {:?}", visibility);
                }
            }

            edges + visible_trees
        }
    }
}
