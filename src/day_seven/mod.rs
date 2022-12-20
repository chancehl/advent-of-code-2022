#[allow(dead_code)]
pub mod problem {
    #[derive(Debug)]
    pub struct ElfDirectory {
        name: String,
        children: Vec<ElfDirectory>,
        files: Vec<ElfFile>,
    }

    #[derive(Debug)]
    struct ElfFile {
        name: String,
        size: u32,
    }

    impl ElfFile {
        pub fn new(name: String, size: u32) -> Self {
            ElfFile { name, size }
        }
    }

    impl ElfDirectory {
        pub fn new(name: String) -> Self {
            ElfDirectory {
                name,
                files: Vec::new(),
                children: Vec::new(),
            }
        }
    }

    pub mod part_one {
        use regex::Regex;

        use super::{ElfDirectory, ElfFile};

        pub fn no_space_on_device(input: &str) -> u32 {
            let terminal_output = input.split("\n").collect::<Vec<&str>>();

            let mut pointer_a = 0;
            let mut pointer_b = pointer_a + 1;

            while pointer_a < terminal_output.len() {
                while !is_cd_command(terminal_output[pointer_b]) {
                    pointer_b = pointer_b + 1;
                }

                let input = terminal_output[pointer_a..pointer_b]
                    .iter()
                    .map(|&s| s.to_owned())
                    .collect::<Vec<String>>();

                let directory = create_directory_from_input(input);

                println!("{:?}", directory);

                pointer_a = pointer_b;
                pointer_b = pointer_a + 1;
            }

            todo!();
        }

        pub fn create_directory_from_input(terminal_output: Vec<String>) -> ElfDirectory {
            let dir_name = terminal_output[0]
                .split(" ")
                .nth(2)
                .expect("Could not parse directory name from terminal output");

            let mut root = ElfDirectory::new(dir_name.to_owned());

            for i in 1..terminal_output.len() {
                let terminal_line = &terminal_output[i];

                if is_list_command(terminal_line.as_ref()) {
                    // if it's a list command just skip (we do nothing with this)
                    continue;
                } else if is_file(terminal_line.as_ref()) {
                    // if it's a file add it to the current node's files array
                    let size = terminal_line
                        .split(" ")
                        .nth(0)
                        .unwrap()
                        .parse::<u32>()
                        .expect("Could not parse u32 from terminal line");

                    let name = terminal_line.split(" ").nth(1).unwrap().to_owned();

                    let new_file = ElfFile::new(name, size);

                    let files = &mut root.files;

                    files.push(new_file);
                } else if is_directory(terminal_line.as_ref()) {
                    // if it's a directory add it to the current node's directories arary
                    let name = terminal_line.split(" ").nth(1).unwrap().to_owned();

                    let new_directory = ElfDirectory::new(name);

                    let directories = &mut root.children;

                    directories.push(new_directory);
                }
            }

            root
        }

        pub fn is_cd_command(input: &str) -> bool {
            Regex::new("\\$\\scd\\s.+").unwrap().is_match(input)
        }

        pub fn is_list_command(input: &str) -> bool {
            Regex::new("\\$\\sls").unwrap().is_match(input)
        }

        pub fn is_file(input: &str) -> bool {
            Regex::new("\\d+\\s[a-zA-Z]+").unwrap().is_match(input)
        }

        pub fn is_directory(input: &str) -> bool {
            Regex::new("dir\\s[a-zA-Z]+").unwrap().is_match(input)
        }
    }
}
