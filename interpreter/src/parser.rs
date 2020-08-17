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
    pub fn get_commands(&self) -> Vec<Command> {
        self.commands
    }

    pub fn from_string(input_string: &String) -> Option<Self> {
        match Self::parse_commands(input_string) {
            Some(commands) => {
                // TODO: parse brackets
                return Some(Self{commands, bracket_pairs: Vec::new()})
            },
            None => return None,
        }
    }

    /// function to parse a string and turn it into a vector of commands
    pub fn parse_string(&mut self, input_string: &String) -> io::Result<bool> {
        match Self::parse_commands(input_string) {
            Some(commands) => {
                self.commands = commands;
                // TODO: parse brackets
                return Ok(true)},
            // returns a system io error in linux
            None => Err(io::Error::from_raw_os_error(5)),
        }
    }

    pub fn get_first_bracket(bracket_list: &Vec<(usize, usize)>, second_bracket: usize) -> Option<usize> {
        for pair in bracket_list.iter() {
            if pair.1 == second_bracket {
                return Some(pair.0);
            }
        }
        None
    }

    pub fn get_second_bracket(bracket_list: &Vec<(usize, usize)>, first_bracket: usize) -> Option<usize> {
        for pair in bracket_list.iter() {
            if pair.0 == first_bracket {
                return Some(pair.1);
            }
        }
        None
    }

    fn find_bracket_pairs(commands: &Vec<Command>) -> Vec<(usize, usize)> {
        let mut pair_vec: Vec<(usize, usize)> = Vec::new();
        let mut bracket_order: Vec<usize> = Vec::new();

        // TODO: fix
        for i in 0..input_vec.len() {
            let c = input_vec[i];
            match c {
                '[' => {
                    pair_vec.push((i, 0));
                    bracket_order.push(pair_vec.len() - 1);
                }
                ']' => {
                    pair_vec[bracket_order[bracket_order.len() - 1]].1 = i;
                    bracket_order.remove(bracket_order.len() - 1);
                }
                _ => (),
            }
        }
        pair_vec
    }

    /// function that turns a string into a vector of commands
    fn parse_commands(string: &String) -> Option<Vec<Command>> {

        let mut next_is_command = false;
        let mut commands: Vec<Command> = Vec::new();

        let mut open_bracket_counter = 0;
        let mut closed_bracket_counter = 0;

        for c in string.chars() {
            match c {
                '>' => commands.push(Command::Brainfuck(BrainfuckCommand::PointerIncrement)),
                '<' => commands.push(Command::Brainfuck(BrainfuckCommand::PointerDecrement)),
                '+' => commands.push(Command::Brainfuck(BrainfuckCommand::DataIncrement)),
                '-' => commands.push(Command::Brainfuck(BrainfuckCommand::DataDecrement)),
                '.' => {
                    commands.push(Command::Brainfuck(BrainfuckCommand::ReturnDataAtPointer))
                }
                ',' => {
                    commands.push(Command::Brainfuck(BrainfuckCommand::WriteDataToPointer))
                }
                '[' => {
                    commands.push(Command::Brainfuck(BrainfuckCommand::OpenBracket));
                    open_bracket_counter += 1;
                },
                ']' => { 
                    commands.push(Command::Brainfuck(BrainfuckCommand::ClosedBracket));
                    closed_bracket_counter += 1;
                },
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

        if open_bracket_counter != closed_bracket_counter {
            None
        } else {
            Some(commands)
        }
    }
}
