use std::collections::HashMap;

use crate::utils;

pub fn main(code: &mut HashMap<u8, &str>, params: Vec<&str>) {

    if params.len() == 0 {
        code_display(code);
    }

}

pub fn code_display(code: &mut HashMap<u8, &str>) {
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = code.get(&x).unwrap_or(&"---");
        println!("{}: {}", address, val);
    }
}
