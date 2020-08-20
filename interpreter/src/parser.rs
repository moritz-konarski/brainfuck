use std::io;

/// enum of all 8 brainfuck commands
pub enum BrainfuckCommand {
    PointerIncrement,
    PointerDecrement,
    DataIncrement,
    DataDecrement,
    ReturnDataAtPointer,
    WriteDataToPointer,
    OpenBracket,
    ClosedBracket,
}

/// enum of all shell commands
pub enum ShellCommand {
    ResetValues,
    PrintString,
    QuitProgram,
}

/// enum for different kinds of commands to collect them
pub enum Command {
    Brainfuck(BrainfuckCommand),
    Shell(ShellCommand),
}

/// struct representing a matching pair of brackets
pub struct BracketPair {
    first_bracket: usize,
    second_bracket: usize,
}

impl BracketPair {
    pub fn set_first_bracket(&self, index: usize) {
        self.first_bracket = index;
    }

    pub fn set_second_bracket(&self, index: usize) {
        self.second_bracket = index;
    }

    pub fn get_first_bracket(&self) -> usize {
        self.first_bracket
    }

    pub fn get_second_bracket(&self) -> usize {
        self.second_bracket
    }
}

/// parser for brainfuck commands
pub struct Parser {
    commands: Vec<Command>,
    bracket_pairs: Vec<(usize, usize)>,
}

impl Parser {
    /// returns a new instance of parser
    pub fn new() -> Self {
        Parser {
            commands: Vec::new(),
            bracket_pairs: Vec::new(),
        }
    }

    /// returns vector of commands
    pub fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn from_string(input_string: &String) -> Option<Self> {
        match Self::parse_commands(input_string) {
            Some(commands) => {
                // TODO: parse brackets
                return Some(Self {
                    commands,
                    bracket_pairs: Vec::new(),
                });
            }
            None => return None,
        }
    }

    /// function to parse a string and turn it into a vector of commands
    pub fn parse_string(&mut self, input_string: &String) -> io::Result<bool> {
        match Self::parse_commands(input_string) {
            Some(commands) => {
                self.commands = commands;
                // TODO: parse brackets
                return Ok(true);
            }
            // returns a system io error in linux
            None => Err(io::Error::from_raw_os_error(5)),
        }
    }

    pub fn get_first_bracket(
        bracket_list: &Vec<(usize, usize)>,
        second_bracket: usize,
    ) -> Option<usize> {
        for pair in bracket_list.iter() {
            if pair.1 == second_bracket {
                return Some(pair.0);
            }
        }
        None
    }

    pub fn get_second_bracket(
        bracket_list: &Vec<(usize, usize)>,
        first_bracket: usize,
    ) -> Option<usize> {
        for pair in bracket_list.iter() {
            if pair.0 == first_bracket {
                return Some(pair.1);
            }
        }
        None
    }

    fn find_bracket_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        let mut bracket_order: Vec<usize> = Vec::new();

        // iterate through all elements with indices
        for (index, element) in self.commands.iter().enumerate() {
            match element {
                Command::Brainfuck(c) => match c {
                    BrainfuckCommand::OpenBracket => {
                        // add open bracket to list
                        pairs.push((index, 0));
                        // add last index of pair_vec to vector
                        // keep track of last opened bracket
                        bracket_order.push(pairs.len() - 1);
                    }
                    BrainfuckCommand::ClosedBracket => {
                        // set second bracket index, at point of the last bracket
                        pairs[bracket_order[bracket_order.len() - 1]].1 = index;
                        // remove last opened bracket
                        bracket_order.remove(bracket_order.len() - 1);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        pairs
    }

    /// function that turns a string into a vector of commands
    fn parse_commands(string: &String) -> Option<Vec<Command>> {
        let mut next_is_command = false;
        let mut commands: Vec<Command> = Vec::new();

        for c in string.chars() {
            match c {
                '>' => commands.push(Command::Brainfuck(BrainfuckCommand::PointerIncrement)),
                '<' => commands.push(Command::Brainfuck(BrainfuckCommand::PointerDecrement)),
                '+' => commands.push(Command::Brainfuck(BrainfuckCommand::DataIncrement)),
                '-' => commands.push(Command::Brainfuck(BrainfuckCommand::DataDecrement)),
                '.' => commands.push(Command::Brainfuck(BrainfuckCommand::ReturnDataAtPointer)),
                ',' => commands.push(Command::Brainfuck(BrainfuckCommand::WriteDataToPointer)),
                '[' => commands.push(Command::Brainfuck(BrainfuckCommand::OpenBracket)),
                ']' => commands.push(Command::Brainfuck(BrainfuckCommand::ClosedBracket)),
                ':' => {
                    next_is_command = true;
                    continue;
                }
                _ => (),
            };

            if next_is_command {
                next_is_command = false;
                match c {
                    'r' => commands.push(Command::Shell(ShellCommand::ResetValues)),
                    'p' => commands.push(Command::Shell(ShellCommand::PrintString)),
                    'q' => commands.push(Command::Shell(ShellCommand::QuitProgram)),
                    _ => (),
                };
            }
        }

        let open_bracket_counter = Self::count_brackets(&commands, BrainfuckCommand::OpenBracket);
        let closed_bracket_counter =
            Self::count_brackets(&commands, BrainfuckCommand::ClosedBracket);

        if open_bracket_counter != closed_bracket_counter {
            None
        } else {
            Some(commands)
        }
    }
}
