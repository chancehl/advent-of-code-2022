pub mod problem {
    use std::str::FromStr;

    use regex::Regex;

    #[derive(Debug)]
    struct ElfDirectory {
        name: String,
        children: Vec<ElfDirectory>,
        files: Vec<ElfFile>,
    }

    #[derive(Debug)]
    struct ElfFile {
        name: String,
        size: u32,
    }

    #[derive(Debug, PartialEq)]
    pub struct ElfTerminalCommand {
        pub command: String,
        pub arg: Option<String>, // all elf terminal commands only have a single argument
    }

    impl FromStr for ElfTerminalCommand {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(" ");

            let command = &parts.nth(1);
            let arg = &parts.nth(1);

            println!("command = {:?}, arg = {:?}", command, arg);

            Ok(ElfTerminalCommand {
                arg: arg.map(|s| s.to_owned()),
                command: command.expect("Could not locate command").to_owned(),
            })
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
        use super::ElfDirectory;

        pub fn no_space_on_device(input: &str) -> u32 {
            let tree = convert_input_to_directory_tree(input);

            todo!();
        }

        pub fn convert_input_to_directory_tree(input: &str) -> u32 {
            let root = ElfDirectory::new("/".to_owned());

            println!("root = {:?}", root);

            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::problem::ElfTerminalCommand;

    #[test]
    fn elf_terminal_command_from_str_test() {
        assert_eq!(
            ElfTerminalCommand::from_str("$ cd sss").unwrap(),
            ElfTerminalCommand {
                arg: Some("/".to_owned()),
                command: "cd".to_owned()
            }
        )
    }
}
