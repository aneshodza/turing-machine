use std::{io, collections::HashMap};

mod cli;
mod utils;

fn main() {
    println!("Welcome to this small virtual CPU simulator!");

    let mut memory: HashMap<u8, u8> = HashMap::new();
    let mut code: HashMap<u8, String> = HashMap::new();
    let mut eye: u8 = 0;

    for x in 0..16 {
        memory.insert(x, 0);
        code.insert(x, String::from("---"));
    }

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.split_whitespace();
        let src_input;
        let src_split;
        
        let cmd :String = utils::parse_input_to_cmd(&input);
        let mut params: Vec<&str> = utils::parse_input_to_vec(input);
        
        if cmd.eq(":step") {
            let default_command = &String::from("throw");
            src_input = code.get(&eye).unwrap_or(default_command).to_string();
            src_split = src_input.split_whitespace();
            params = utils::parse_input_to_vec(src_split);
            eye = (eye+1).rem_euclid(16);
        }

        cli::main(
            params.remove(0).to_string(),
            &mut memory,
            &mut code,
            params,
            eye
        );
    }
}
