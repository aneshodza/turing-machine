use std::{io, collections::HashMap};

mod cli;
mod utils;

fn main() {
    println!("Welcome to this small virtual CPU simulator!");

    let mut memory: HashMap<u8, u8> = HashMap::new();
    let mut code: HashMap<u8, String> = HashMap::new();
    for x in 0..16 {
        memory.insert(x, 0);
        code.insert(x, String::from("---"));
    }

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut input = input.split_whitespace();
        
        let cmd :String = String::from(input.next().unwrap_or(":h"));
        let params: Vec<&str> = input.collect();

        if cmd.chars().nth(0).unwrap().eq(&":".chars().nth(0).unwrap()) {
            cli::main(
                cmd,
                &mut memory,
                &mut code,
                params
            )
        }
    }
}
