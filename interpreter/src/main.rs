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
 *  Hello World:
 *  ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
 */
use std::char;
use std::io::{self, Write};

// TODO: 
//  - make data type for printing uniform -- figure out how to do it better
//  - make all the functions safer and add error messages
//  - create a bracket pair struct with methods that takes care of the pairs

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
        if self.index >= 1 {
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

        //println!("{:?}", bracket_pairs);

        /*
        for pair in bracket_pairs.iter() {
            println!("open bracket {} goes with {}", pair.0, pair.1);
            println!(
                "open bracket {} goes with {}",
                get_first_bracket(&bracket_pairs, pair.1),
                get_second_bracket(&bracket_pairs, pair.0)
            );
        }
        */

        let mut output_string: Vec<char> = Vec::new();
        let mut printing = false;

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
                '.' => {
                    printing = true;
                    output_string.push(mem_space.get_data());
                }
                // TODO: make input function
                ',' => {
                    println!("Enter 1 character:");
                    let mut read_input = String::new();
                    io::stdin()
                        .read_line(&mut read_input)
                        .expect("Error: could not read input");
                    let c: char = match read_input.chars().next() {
                        Some(c) => c,
                        None => 0 as char,
                    };
                    mem_space.write_data(c);
                }
                // '[' if the byte at the pointer is 0, jump to ']', else to next command
                '[' => {
                    if mem_space.is_data_zero() {
                        //println!("open bracket at {}", i - 1);
                        i = get_second_bracket(&bracket_pairs, i - 1);
                        //println!("jump to {}", i);
                    }
                }
                // ']' if the data pointer is not 0, jump to '[', else to next command
                ']' => {
                    if !mem_space.is_data_zero() {
                        //println!("closed bracket at {}", i - 1);
                        i = get_first_bracket(&bracket_pairs, i - 1);
                        //println!("jump to {}", i);
                    }
                }
                ':' => {
                    is_command = true;
                    continue;
                }
                _ => (),
            };

            if printing {
                printing = false;
                for c in output_string.iter() {
                    print!("{}", c);
                }
                match io::stdout().flush() {
                    Ok(_) => (),
                    Err(_) => eprintln!("Error: failed to print output"),
                };
            }

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

fn get_first_bracket(bracket_list: &Vec<(usize, usize)>, second_bracket: usize) -> usize {
    for pair in bracket_list.iter() {
        if pair.1 == second_bracket {
            return pair.0;
        }
    }
    0
}

fn get_second_bracket(bracket_list: &Vec<(usize, usize)>, first_bracket: usize) -> usize {
    for pair in bracket_list.iter() {
        if pair.0 == first_bracket {
            return pair.1;
        }
    }
    0
}
