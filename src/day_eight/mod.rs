pub mod problem {
    use std::str::FromStr;

    pub struct Graph<T> {
        pub elements: Vec<Vec<T>>,
    }

    impl Graph<u32> {
        pub fn get_adjacent_nodes(row: usize, col: usize) -> Vec<u32> {
            todo!()
        }

        pub fn count_edges(&self) -> u32 {
            let rows = self.elements.len();
            let cols = self.elements[0].len();

            todo!()
        }
    }

    impl FromStr for Graph<u32> {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Graph {
                elements: s
                    .split("\n")
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
            })
        }
    }

    pub mod part_one {
        use std::str::FromStr;

        use super::Graph;

        pub fn treetop_tree_house(input: &str) -> u32 {
            let graph = Graph::from_str(input).expect("Could not convert input to Graph type");

            let rows = graph.elements.len();
            let cols = graph.elements[0].len();

            for i in 0..rows {
                for j in 0..cols {
                    println!("graph[i][j] = {:?}", graph.elements[i][j]);
                }
            }

            todo!()
        }
    }
}
