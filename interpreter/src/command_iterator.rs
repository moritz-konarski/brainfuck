// TODO: create command iterator
// - pub next()
// - jump_to_index
// - pub jump_to_matching_brace
use crate::parser::{Parser, Command, ShellCommand, BrainfuckCommand};

pub struct CommandInterator {
    parser: Parser,
}

impl CommandInterator {
    pub fn next(&self) /*-> Command*/ {

    }

    pub fn has_next(&self) -> bool {
        false
    }

    fn jump_to_index(&self) {

    }

    pub fn jump_to_matching_brace(&self) {

    }
}