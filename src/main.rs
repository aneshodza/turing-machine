use std::{io, collections::HashMap, process::exit};

fn main() {
    println!("Welcome to this small virtual CPU simulator!");

    let mut memory: HashMap<String, u8> = HashMap::new();
    for x in 0..16 {
        memory.insert(get_hex_string(x), 0);
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
            list_storage(memory_ref);
        }

    }
}

fn get_hex_string(num: u8) -> String {
    if num > 16 {
        return String::from(format!("0x{:X}", num));
    } else {
        return String::from(format!("0x0{:X}", num));
    }
}

fn list_storage(memory_ref: &HashMap<String, u8>) {
    for x in 0..16 {
        let address = get_hex_string(x);
        let val = memory_ref.get(&address).unwrap_or(&0);
        println!("{}: {}", address, get_hex_string(*val))
    }
}
