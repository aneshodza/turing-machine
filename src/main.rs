use std::{io, collections::HashMap, process::exit, borrow::Borrow};

mod cli_funcs;
mod utils;

fn main() {
    println!("Welcome to this small virtual CPU simulator!");

    let mut memory: HashMap<String, u8> = HashMap::new();
    let mut code: HashMap<String, &str> = HashMap::new();
    for x in 0..16 {
        let addy: String = utils::get_hex_string(x);
        memory.insert(addy.clone(), 0);
        code.insert(addy.clone(), "---");
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
            cli_funcs::list_storage(memory_ref, params);
        } else if cmd.eq(":src") {
            cli_funcs::list_code(code_ref, params)
        }
        else {
            cli_funcs::help();
        }
    }
}
