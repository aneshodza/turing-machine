use std::{collections::HashMap, process::exit};

use crate::utils;

pub fn memory_funcs(memory_ref: &HashMap<u8, u8>, params: Vec<&str>) -> HashMap<u8, u8> {
    let memory_clone = memory_ref.clone();
    if params.len() == 0 {
        for x in 0..16 {
            let address = utils::get_hex_string(x);
            let val = memory_clone.get(&x).unwrap_or(&0);
            println!("{}: {}", address, utils::get_hex_string(*val))
        }
        return memory_clone;
    }

    if params[0].eq("write") {
    }

    return memory_clone;

}

pub fn code_funcs<'a>(code_ref: &HashMap<u8, &'a str>, params: Vec<&str>) -> HashMap<u8, &'a str>{
    let code_clone = code_ref.clone();
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = code_ref.get(&x).unwrap_or(&"---");
        println!("{}: {}", address, val);
        return code_clone;
    }

    return code_clone;
}

pub fn exit_code(params: Vec<&str>) {
    exit(0);
}

pub fn help() {
    println!("For help, read the GitHub docs")
}
