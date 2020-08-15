use crate::parser::{BrainfuckCommand, Command, ShellCommand};

/// struct for a brainfuck interpreter
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret_commands(parsed_commands: Vec<Command>) {
        for command in parsed_commands {
            match command {
                Command::Shell(c) => match c {
                    ShellCommand::ResetValues => (),
                    ShellCommand::PrintString => (),
                    ShellCommand::QuitProgram => (),
                },
                Command::Brainfuck(c) => match c {
                    BrainfuckCommand::PointerIncrement => (),
                    BrainfuckCommand::PointerDecrement => (),
                    BrainfuckCommand::DataIncrement => (),
                    BrainfuckCommand::DataDecrement => (),
                    BrainfuckCommand::ReturnDataAtPointer => (),
                    BrainfuckCommand::WriteDataToPointer => (),
                    BrainfuckCommand::OpenBracket => (),
                    BrainfuckCommand::ClosedBracket => (),
                },
            }
        }
    }
}
