// TODO: create command iterator
// - pub next()
// - jump_to_index
// - pub jump_to_matching_brace
use crate::bracket_pair::BracketPair;
use crate::enums::{BrainfuckCommand, Command, ShellCommand};

/// iterator-esque struct that iterates over the supplied commands
pub struct CommandInterator {
    index: usize,
    max_len: usize,
    commands: Vec<Command>,
    bracket_pairs: Vec<BracketPair>,
}

impl CommandInterator {
    /// returns a new instance of command iterator
    pub fn new(commands: Vec<Command>, bracket_pairs: Vec<BracketPair>) -> Self {
        Self {
            index: 0,
            max_len: commands.len(),
            commands,
            bracket_pairs,
        }
    }

    /// returns the next element in the command list
    pub fn next(&self) -> Option<Command> {
        if self.index < self.max_len {
            self.index += 1;
            Some(self.commands[self.index - 1])
        } else {
            None
        }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.max_len
    }

    pub fn jump_to_matching_bracket(&self) -> bool {
        match self.commands[self.index] {
            Command::Brainfuck(comm) => match comm {
                BrainfuckCommand::OpenBracket => {
                    // TODO: come up with a way to couple the brackets together that
                    // does not suck
                }
                BrainfuckCommand::ClosedBracket => {
                    // TODO: come up with a way to couple the brackets together that
                    // does not suck
                }
                _ => return false,
            },
            _ => return false,
        }
    }
}
