use crate::enums::{MemoryUnit, MemoryUnitType};
use std::char;

/// struct that stores the pointer location and values of the memory space
pub struct MemorySpace {
    pointer_location: usize,
    memory: Vec<MemoryUnit>,
}

impl MemorySpace {
    /// takes the type of memory and returns a MemorySpace
    pub fn new(memory_type: MemoryUnitType) -> Self {
        let mut mem = Self {
            pointer_location: 0,
            memory: Vec::new(),
        };
        match memory_type {
            MemoryUnitType::Int8Bit => mem.memory.push(MemoryUnit::Int8Bit(0)),
            MemoryUnitType::Int32Bit => mem.memory.push(MemoryUnit::Int32Bit(0)),
        };
        mem
    }

    /// action for '>', incrementing the pointer or moving it right
    pub fn pointer_increment(&mut self) {
        self.pointer_location += 1;

        if self.pointer_location >= self.memory.len() {
            match self.memory[self.pointer_location - 1] {
                MemoryUnit::Int8Bit(_) => {
                    self.memory.push(MemoryUnit::Int8Bit(0));
                }
                MemoryUnit::Int32Bit(_) => {
                    self.memory.push(MemoryUnit::Int32Bit(0));
                }
            };
        }
    }

    /// action for '>', decrementing the pointer or moving it left
    pub fn pointer_decrement(&mut self) -> Option<usize> {
        if self.pointer_location >= 1 {
            self.pointer_location -= 1;
            Some(self.pointer_location)
        } else {
            None
        }
    }

    /// action for '+', incrementing the pointer at the current location
    pub fn data_increment(&mut self) {
        match self.memory[self.pointer_location] {
            MemoryUnit::Int8Bit(x) => {
                self.memory[self.pointer_location] = MemoryUnit::Int8Bit(x + 1);
            }
            MemoryUnit::Int32Bit(x) => {
                self.memory[self.pointer_location] = MemoryUnit::Int32Bit(x + 1);
            }
        };
    }

    /// action for '-', decrementing the pointer at the current location
    pub fn data_decrement(&mut self) {
        match self.memory[self.pointer_location] {
            MemoryUnit::Int8Bit(x) => {
                self.memory[self.pointer_location] = MemoryUnit::Int8Bit(x - 1);
            }
            MemoryUnit::Int32Bit(x) => {
                self.memory[self.pointer_location] = MemoryUnit::Int32Bit(x - 1);
            }
        };
    }

    /// action for '.', return the ascii variant of the number
    /// at the current pointer position
    pub fn get_data_as_char(&self) -> Option<char> {
        let mut c: u32 = 0;
        match self.memory[self.pointer_location] {
            MemoryUnit::Int8Bit(x) => {
                c = x as u32;
            }
            MemoryUnit::Int32Bit(x) => {
                c = x;
            }
        };
        char::from_u32(c)
    }

    /// action for ',', store one character in the current pointer position
    pub fn write_data(&mut self, c: char) {
        match self.memory[self.pointer_location] {
            MemoryUnit::Int8Bit(_) => {
                self.memory[self.pointer_location] = MemoryUnit::Int8Bit(c as u8);
            }
            MemoryUnit::Int32Bit(_) => {
                self.memory[self.pointer_location] = MemoryUnit::Int32Bit(c as u32);
            }
        };
    }

    /// reset the pointer and memory to standard states
    pub fn reset(&mut self) {
        self.pointer_location = 0;
        self.memory = Vec::new();
    }

    /// checks whether the pointer is pointing at 0
    pub fn is_data_at_pointer_zero(&self) -> bool {
        match self.memory[self.pointer_location] {
            MemoryUnit::Int8Bit(x) => x == 0,
            MemoryUnit::Int32Bit(x) => x == 0,
        }
    }
}
