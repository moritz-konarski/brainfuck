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
use std::char;
use std::io::{self, Write};

const ARRAY_SIZE: usize = 30_000;

struct MemorySpace {
    index: usize,
    array: [u8; ARRAY_SIZE],
}

impl MemorySpace {
    // action for '>'
    fn pointer_increment(&mut self) {
        if self.index < ARRAY_SIZE - 1 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

    // action for '<'
    fn pointer_decrement(&mut self) {
        if self.index > 1 {
            self.index -= 1;
        } else {
            self.index = ARRAY_SIZE - 1;
        }
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
    fn get_data(&self) -> char {
        match char::from_u32(self.array[self.index] as u32) {
            Some(c) => c,
            None => ' ',
        }
    }

    // action for ','
    fn write_data(&mut self, c: char) {
        self.array[self.index] = c as u8;
    }

    fn reset(self) -> MemorySpace {
        MemorySpace {
            index: 0,
            array: [0; ARRAY_SIZE],
        }
    }

    fn new() -> MemorySpace {
        MemorySpace {
            index: 0,
            array: [0; ARRAY_SIZE],
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_array(&self) -> &[u8] {
        &self.array[..25]
    }

    fn is_data_zero(&self) -> bool {
        self.array[self.index] == 0
    }
}

fn main() {
    println!("Brainfuck Interpreter");

    let mut input: String = String::new();
    let mut mem_space: MemorySpace = MemorySpace::new();
    let mut quit: bool = false;
    let mut is_command: bool = false;

    loop {
        print!("bf> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Error: writing to screen failed");
                break;
            }
        }

        io::stdin()
            .read_line(&mut input)
            .expect("Error: could not read input");

        input = input.trim().to_string();

        if !check_line_syntax(&input) {
            eprintln!("Error: brackets do not match");
            break;
        }

        let input: Vec<_> = input.chars().collect();

        let bracket_pairs = get_bracket_pairs(&input);
        println!("{:?}", bracket_pairs);
        let mut open_bracket_index = 0;
        let mut closed_bracket_index = 0;
        let mut i = 0;

        loop {
            if i >= input.len() {
                break;
            }

            let c = input[i];
            i += 1;

            match c {
                '>' => mem_space.pointer_increment(),
                '<' => mem_space.pointer_decrement(),
                '+' => mem_space.data_increment(),
                '-' => mem_space.data_decrement(),
                '.' => println!("{}", mem_space.get_data()),
                // TODO: make input function
                ',' => mem_space.write_data('c'),
                /*
                '[' => {
                    if mem_space.is_data_zero() {
                        i = bracket_pairs[open_bracket_index].1;                                              
                        open_bracket_index += 1;
                    }
                },
                ']' => {
                    if !mem_space.is_data_zero() {
                        i = bracket_pairs[closed_bracket_index].0;                                              
                        closed_bracket_index += 1;
                    }
                },
                */
                ':' => {is_command = true; continue;},
                _ => (),
            };

            if is_command {
                is_command = false;
                match c {
                'r' => mem_space = mem_space.reset(),
                'q' => {
                    mem_space = mem_space.reset();
                    quit = true;
                }
                _ => (),
                };
            };
        }

        println!("{:#?}; {:?}", mem_space.get_index(), mem_space.get_array());
        if quit {
            break;
        }
    }
}

fn check_line_syntax(input: &String) -> bool {
    let mut open_bracket_counter: u16 = 0;
    let mut closed_bracket_counter: u16 = 0;
    for c in input.chars() {
        match c {
            '[' => open_bracket_counter += 1,
            ']' => closed_bracket_counter += 1,
            _ => (),
        }
    }
    open_bracket_counter == closed_bracket_counter
}

fn get_bracket_pairs(input_vec: &Vec<char>) -> Vec<(usize, usize)> {
    let mut pair_vec: Vec<(usize, usize)> = Vec::new();
    let mut bracket_order: Vec<usize> = Vec::new();

    for i in 0..input_vec.len() {
        let c = input_vec[i];
        //println!("{:?}", pair_vec);
        match c {
            '[' => {
                pair_vec.push((i, 0));
                bracket_order.push(pair_vec.len()-1);
            },
            ']' => {
                pair_vec[bracket_order[bracket_order.len()-1]].1 = i;
                bracket_order.remove(bracket_order.len() - 1);
            },
            _ => (),
        }        
    }
    pair_vec
}