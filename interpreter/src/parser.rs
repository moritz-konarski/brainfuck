use crate::bracket_pair::BracketPair;
use crate::enums::{BrainfuckCommand, Command, ShellCommand};

/// parser for brainfuck commands
pub struct Parser {
    commands: Vec<Command>,
    bracket_pairs: Vec<BracketPair>,
}

impl Parser {
    /// returns a new instance of parser
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            bracket_pairs: Vec::new(),
        }
    }

    /// returns an Option of parser that tried to parse the provided string
    /// Option is returned because the parse could fail
    pub fn from_string(input_string: &String) -> Option<Self> {
        match Self::parse_commands(input_string) {
            Some(commands) => {
                let bracket_pairs = Self::find_bracket_pairs(&commands);
                return Some(Self {
                    commands,
                    bracket_pairs,
                });
            }
            None => return None,
        }
    }

    /// function to parse a string and turn it into a vector of commands
    /// returns a Result type because the parsing could fail
    pub fn parse_string(&mut self, input_string: &String) -> Option<bool> {
        match Self::parse_commands(input_string) {
            Some(commands) => {
                self.commands = commands;
                self.bracket_pairs = Self::find_bracket_pairs(&commands);
                return Some(true);
            }
            None => None,
        }
    }

    /// returns vector of commands
    pub fn get_commands(&self) -> Vec<Command> {
        self.commands
    }

    /// returns vector of bracket pairs
    pub fn get_bracket_pairs(&self) -> Vec<BracketPair> {
        self.bracket_pairs
    }

    /// function that finds all the bracket pairs in the command vector
    // TODO: come up with a good way to couple the brackets together that does not suck
    // it should also work with the standard enum of BrainfuckCommands
    fn find_bracket_pairs(commands: &Vec<Command>) -> Vec<BracketPair> {
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        let mut bracket_order: Vec<usize> = Vec::new();

        // iterate through all elements with indices
        for (index, element) in commands.iter().enumerate() {
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

        let mut bracket_pairs: Vec<BracketPair> = Vec::new();

        for index_pair in pairs {
            bracket_pairs.push(BracketPair::from_indeces(index_pair));
        }

        bracket_pairs
    }

    /// function that turns a string into a vector of commands
    fn parse_commands(string: &String) -> Option<Vec<Command>> {
        let mut open_bracket_count = 0;
        let mut closed_bracket_count = 0;

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
                '[' => {
                    commands.push(Command::Brainfuck(BrainfuckCommand::OpenBracket));
                    open_bracket_count += 1;
                }
                ']' => {
                    commands.push(Command::Brainfuck(BrainfuckCommand::ClosedBracket));
                    closed_bracket_count += 1;
                }
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

        if open_bracket_count != closed_bracket_count {
            None
        } else {
            Some(commands)
        }
    }
}
