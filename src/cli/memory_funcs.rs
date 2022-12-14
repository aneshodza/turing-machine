use std::{collections::HashMap};

use crate::utils;

pub fn main(memory: &mut HashMap<u8, u8>, params: Vec<&str>) {

    if params.len() == 0 {
        memory_display(memory);
    } else if params[0] == "write" {
        memory_write(memory, params);
    } else {

    }
}

pub fn memory_display(memory: &mut HashMap<u8, u8>) {
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = memory.get(&x).unwrap_or(&0);
        println!("{}: {}", address, utils::get_hex_string(*val));
    }
}

pub fn memory_write(memory: &mut HashMap<u8, u8>, params: Vec<&str>) {

    if params.len() != 3 {
        utils::generic_error("Wrong amount of params");
        return;
    } else if !!!utils::is_u8(params[1]) || !!!utils::is_u8(params[2]) {
        utils::generic_error("Params aren't in u8");
        return;
    }
    memory.insert(
        utils::str_to_u8(params[1]),
        utils::str_to_u8(params[2])
    );
    println!("Wrote value");

    return;
}
