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

/// Enum for the different type of storage cell sizes
/// classic size is 8 bits unsigned (unsigned Char), more modern variation is Int (32-bit unsigned)
#[derive(Clone)]
pub enum MemoryUnit {
    Int8Bit(u8),
    Int32Bit(u32),
}

/// Enum for the different type of storage cell sizes to be set by user
/// classic size is 8 bits unsigned (unsigned Char), more modern variation is Int (32-bit unsigned)
pub enum MemoryUnitType {
    Int8Bit,
    Int32Bit,
}
