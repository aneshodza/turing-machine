use std::{collections::HashMap, process::exit};

use crate::utils;

pub fn list_storage(memory_ref: &HashMap<String, u8>, params: Vec<&str>) {
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = memory_ref.get(&address).unwrap_or(&0);
        println!("{}: {}", address, utils::get_hex_string(*val))
    }
}

pub fn list_code(code_ref: &HashMap<String, &str>, params: Vec<&str>) {
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = code_ref.get(&address).unwrap_or(&"---");
        println!("{}: {}", address, val);
    }
}

pub fn exit_code(params: Vec<&str>) {
    exit(0);
}

pub fn help() {
    println!("For help, read the GitHub docs")
}
