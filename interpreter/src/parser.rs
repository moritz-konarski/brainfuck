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
    bracket_pairs: Vec<(usize, usize)>,
}

impl Parser {
    /// returns a new instance of parser
    pub fn new() -> Self {
        Parser {
            bracket_pairs: Vec::new(),
        }
    }

    /// function to parse a string and turn it into a vector of commands
    pub fn parse_string(&mut self, input_string: &String) -> Option<Vec<Command>> {
        let mut next_is_command = false;
        let mut parsed_commands: Vec<Command> = Vec::new();

        for c in input_string.chars() {
            match c {
                '>' => parsed_commands.push(Command::Brainfuck(BrainfuckCommand::PointerIncrement)),
                '<' => parsed_commands.push(Command::Brainfuck(BrainfuckCommand::PointerDecrement)),
                '+' => parsed_commands.push(Command::Brainfuck(BrainfuckCommand::DataIncrement)),
                '-' => parsed_commands.push(Command::Brainfuck(BrainfuckCommand::DataDecrement)),
                '.' => {
                    parsed_commands.push(Command::Brainfuck(BrainfuckCommand::ReturnDataAtPointer))
                }
                ',' => {
                    parsed_commands.push(Command::Brainfuck(BrainfuckCommand::WriteDataToPointer))
                }
                '[' => parsed_commands.push(Command::Brainfuck(BrainfuckCommand::OpenBracket)),
                ']' => parsed_commands.push(Command::Brainfuck(BrainfuckCommand::ClosedBracket)),
                ':' => {
                    next_is_command = true;
                    continue;
                }
                _ => (),
            };

            if next_is_command {
                next_is_command = false;
                match c {
                    'r' => parsed_commands.push(Command::Shell(ShellCommand::ResetValues)),
                    'p' => parsed_commands.push(Command::Shell(ShellCommand::PrintString)),
                    'q' => parsed_commands.push(Command::Shell(ShellCommand::QuitProgram)),
                    _ => (),
                };
            }
        }

        //if !check_matching_brackets(&parsed_commands) {
        //    return None;
        //}

        Some(parsed_commands)
    }

    /// check the commands for matching numbers of strings
    fn check_matching_brackets(commands: &Vec<Command>) -> bool {
        let mut open_bracket_counter = 0;
        let mut closed_bracket_counter = 0;
        for c in commands {
            match c {
                Command::Brainfuck(x) => match x {
                    BrainfuckCommand::OpenBracket => open_bracket_counter += 1,
                    BrainfuckCommand::ClosedBracket => closed_bracket_counter += 1,
                    _ => (),
                },
                _ => (),
            };
        }
        open_bracket_counter == closed_bracket_counter
    }
}
