use std::{collections::HashMap, process::exit};

mod memory_funcs;
mod code_funcs;
use crate::utils;

pub fn main(
    cmd: String,
    memory: &mut HashMap<u8, u8>,
    code: &mut HashMap<u8, String>,
    params: Vec<&str>,
    eye: u8
) {
    if cmd.eq("---") {
        println!("nothing to be executed ma homes");
    } else if cmd.eq(":mem") {
        memory_funcs::main(memory, params);
    } else if cmd.eq(":src") {
        code_funcs::main(code, params, eye);
    } else if cmd.eq(":q") {
        exit(0);
    } else if cmd.eq(":eye") {
        println!(
            "Current address of eye: {}",
            utils::get_hex_string(eye)
        );
    }
    else {
        if !!!cmd.eq(":help") {
            println!("Didn't recognize command");
        }
        println!("For help, read the GitHub docs")
    }
}
