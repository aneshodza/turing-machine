use std::{io, collections::HashMap, process::exit};

mod cli_funcs;
mod utils;

fn main() {
    println!("Welcome to this small virtual CPU simulator!");

    let mut memory: HashMap<String, u8> = HashMap::new();
    for x in 0..16 {
        memory.insert(utils::get_hex_string(x), 0);
    }

    loop {
        let memory_ref = &memory;
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut input = input.split_whitespace();
        
        let cmd :String = String::from(input.next().unwrap_or(":h"));
        let params: Vec<&str> = input.collect();
        
        if cmd.eq(":q") {
            exit(0);
        } else if cmd.eq(":mem") {
            cli_funcs::list_storage(memory_ref);
        }

    }
}
