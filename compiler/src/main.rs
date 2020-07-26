/*
 * This is a brainfuck compiler
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

fn main() {
    println!("Brainfuck Compiler");
}
