/*
 * This is a brainfuck interpreter
 *
 * brainfuck has only 8 instructions:
 *  - '>' increment the data pointer
 *  - '<' decrement the data pointer
 *  - '+' increment the byte at the data pointer
 *  - '-' decrement the byte at the data pointer
 *  - '.' output the byte at the data pointer
 *  - ',' accept one byte of input, storing it at the data pointer
 *  - '[' if the byte at the pointer is 0, jump to ']', else to next command 
 *  - ']' if the data pointer is not 0, jump to '[', else to next command
 *  
 *  standard implementation has an array of 30,000 cells of u8
 *  what happens when the pointer goes out of bounds -- error? -- best solution
 */
use std::io;
use std::char;

struct MemorySpace {
    index: usize,
    array: [u32; 30_000],
}

impl MemorySpace{
    // action for '>'
    fn pointer_increment(&mut self) {
        self.index += 1;
    }

    // action for '<'
    fn pointer_decrement(&mut self) {
        self.index -= 1;
    }

    // action for '+'
    fn data_increment(&mut self) {
        self.array[self.index] += 1;
    }

    // action for '-'
    fn data_decrement(&mut self) {
        self.array[self.index] -= 1;
    }

    // action for '.'
    fn output_data(&self) -> char {
        match char::from_u32(self.array[self.index]) {
            Some(c) => c,
            None => ' ',
        }
    }

    // action for ','
    fn write_data(&mut self, c: char) {
        self.array[self.index] = c as u32;
    }

    fn reset(self) -> MemorySpace {
        MemorySpace {
            index: 0,
            array: [0; 30_000],
        }
    }

    fn new() -> MemorySpace {
        MemorySpace {
            index: 0,
            array: [0; 30_000],
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_array(&self) -> &[u32] {
        &self.array[..100]
    }
}

fn main() {
    println!("Brainfuck Interpreter");

    let mut input = String::new();
    let mut mem_space = MemorySpace::new();

    loop {
        io::stdin().read_line(&mut input)
            .expect("Could not read input");

        for c in input.chars() {
            match c {
                '>' => mem_space.pointer_increment(),
                '<' => mem_space.pointer_decrement(),
                '+' => mem_space.data_increment(),
                '-' => mem_space.data_decrement(),
                '.' => mem_space.output_data(),
                ',' => mem_space.write_data(c),
                _ => (),
            }
        }
        
        println!("  {}", input);
        println!("{:#?}; {:?}", mem_space.get_index(), mem_space.get_array());
    }
}

