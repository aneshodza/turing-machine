use std::{io, collections::HashMap, process::exit, borrow::Borrow};

mod cli_funcs;
mod utils;

fn main() {
    println!("Welcome to this small virtual CPU simulator!");

    let mut memory: HashMap<u8, u8> = HashMap::new();
    let mut code: HashMap<u8, &str> = HashMap::new();
    for x in 0..16 {
        memory.insert(x, 0);
        code.insert(x, "---");
    }

    loop {
        let memory_ref = &memory;
        let code_ref = &code;
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut input = input.split_whitespace();
        
        let cmd :String = String::from(input.next().unwrap_or(":h"));
        let params: Vec<&str> = input.collect();
        
        if cmd.eq(":q") {
            cli_funcs::exit_code(params);
        } else if cmd.eq(":mem") {
            memory = cli_funcs::memory_funcs(memory_ref, params);
        } else if cmd.eq(":src") {
            code = cli_funcs::code_funcs(code_ref, params)
        }
        else {
            cli_funcs::help();
        }
    }
}
