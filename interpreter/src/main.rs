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
 *  Hello World:
 *  ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
 */
use std::char;
use std::io::{self, Write};

// TODO:
//  - make all the functions safer and add error messages
//  - create a bracket pair struct with methods that takes care of the pairs
//  - make state machine to make errors and exits simpler
//  - make functionality where a code file can be supplied to execute more directly
//  - create enum with commands

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
    fn get_data_as_char(&self) -> Option<char> {
        char::from_u32(self.array[self.index] as u32)
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

    let mut output_string: String = String::new();

    // struct simulating the braifuck memory layout and functions
    let mut mem_space = MemorySpace::new();

    // whether the program should quit
    let mut quit = false;

    let mut output_mode_immediate = false;

    // if the next character is a command or not
    let mut is_command = false;

    loop {
        print!("bf> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Error: writing to screen failed");
                break;
            }
        }

        // input string
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error: could not read input");

        input = input.trim().to_string();

        if !do_brackets_match(&input) {
            eprintln!("Error: brackets do not match");
            break;
        }

        //println!("{}", input);

        let input: Vec<_> = input.chars().collect();
        let bracket_pairs = get_bracket_pairs(&input);

        let mut i = 0;

        loop {
            if i >= input.len() {
                println!("");
                break;
            }

            let c = input[i];

            i += 1;

            match c {
                '>' => mem_space.pointer_increment(),
                '<' => mem_space.pointer_decrement(),
                '+' => mem_space.data_increment(),
                '-' => mem_space.data_decrement(),
                '.' => match mem_space.get_data_as_char() {
                    Some(c) => {
                        output_string.push(c);
                        if output_mode_immediate {
                            print!("{}", c);
                            match io::stdout().flush() {
                                Ok(_) => (),
                                Err(_) => {
                                    eprintln!("Error: writing to screen failed");
                                    break;
                                }
                            }
                        }
                    }
                    None => break,
                },
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
                '[' => {
                    if mem_space.is_data_zero() {
                        let index = get_second_bracket(&bracket_pairs, i - 1);
                        match index {
                            Some(x) => i = x,
                            None => eprintln!("Error returning bracket pair"),
                        };
                    }
                }
                ']' => {
                    if !mem_space.is_data_zero() {
                        let index = get_first_bracket(&bracket_pairs, i - 1);
                        match index {
                            Some(x) => i = x,
                            None => eprintln!("Error returning bracket pair"),
                        };
                    }
                }
                ':' => {
                    is_command = true;
                    continue;
                }
                _ => (),
            };

            if is_command {
                is_command = false;
                match c {
                    'r' => mem_space = mem_space.reset(),
                    'l' => println!("{:#?}; {:?}", mem_space.get_index(), mem_space.get_array()),
                    'p' => {
                        println!("{}", output_string);
                        output_string = String::new();
                    }
                    'o' => output_mode_immediate = !output_mode_immediate,
                    'q' => {
                        quit = true;
                        break;
                    }
                    _ => (),
                };
            };
        }

        //println!("{:#?}; {:?}", mem_space.get_index(), mem_space.get_array());
        if quit {
            break;
        }
    }
}

fn do_brackets_match(input: &String) -> bool {
    let mut open_bracket_counter = 0;
    let mut closed_bracket_counter = 0;

    for c in input.chars() {
        match c {
            '[' => open_bracket_counter += 1,
            ']' => closed_bracket_counter += 1,
            _ => (),
        }
    }

    // does the number of brackets match
    open_bracket_counter == closed_bracket_counter
}

fn get_bracket_pairs(input_vec: &Vec<char>) -> Vec<(usize, usize)> {
    let mut pair_vec: Vec<(usize, usize)> = Vec::new();
    let mut bracket_order: Vec<usize> = Vec::new();

    for i in 0..input_vec.len() {
        let c = input_vec[i];
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

fn get_first_bracket(bracket_list: &Vec<(usize, usize)>, second_bracket: usize) -> Option<usize> {
    for pair in bracket_list.iter() {
        if pair.1 == second_bracket {
            return Some(pair.0);
        }
    }
    None
}

fn get_second_bracket(bracket_list: &Vec<(usize, usize)>, first_bracket: usize) -> Option<usize> {
    for pair in bracket_list.iter() {
        if pair.0 == first_bracket {
            return Some(pair.1);
        }
    }
    None
}
