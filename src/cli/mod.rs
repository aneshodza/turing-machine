use std::{collections::HashMap, process::exit};

mod memory_funcs;
mod code_funcs;


pub fn main(cmd: String, memory: &mut HashMap<u8, u8>, code: &mut HashMap<u8, String>, params: Vec<&str> ) {
    if cmd.eq(":mem") {
        memory_funcs::main(memory, params);
    } else if cmd.eq(":src") {
        code_funcs::main(code, params);
    } else if cmd.eq(":q") {
        exit(0);
    } else {
        if !!!cmd.eq(":help") {
            println!("Didn't recognize command");
        }
        println!("For help, read the GitHub docs")
    }
}
