pub mod problem {
    use std::str::FromStr;

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

    #[derive(Debug, PartialEq)]
    pub struct ElfTerminalCommand {
        pub command: String,
        pub arg: Option<String>, // all elf terminal commands only have a single argument
    }

    impl FromStr for ElfTerminalCommand {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(" ");

            let _prefix = &parts.nth(0);
            let command = &parts.nth(0);
            let arg = &parts.nth(0);

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
        use std::str::FromStr;

        use regex::Regex;

        use super::ElfTerminalCommand;

        pub fn no_space_on_device(input: &str) -> u32 {
            let file_regex = Regex::new("\\d+\\s+[a-zA-Z]+").unwrap();
            let dir_regex = Regex::new("dir\\s+[a-zA-Z]+").unwrap();

            let raw_commands = input.split("\n").collect::<Vec<&str>>();

            // grab navigation commands to build tree
            let navigation_commands = raw_commands
                .iter()
                .filter(|l| !file_regex.is_match(l) && !dir_regex.is_match(l))
                .map(|l| ElfTerminalCommand::from_str(l).unwrap())
                .collect::<Vec<ElfTerminalCommand>>();

            for index in 0..navigation_commands.len() {
                println!(
                    "navigation command {:?} => {:?}",
                    index, navigation_commands[index]
                )
            }

            todo!();
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
            ElfTerminalCommand::from_str("$ cd dir").unwrap(),
            ElfTerminalCommand {
                arg: Some("dir".to_owned()),
                command: "cd".to_owned()
            }
        );

        assert_eq!(
            ElfTerminalCommand::from_str("$ ls").unwrap(),
            ElfTerminalCommand {
                arg: None,
                command: "ls".to_owned()
            }
        );
    }
}
