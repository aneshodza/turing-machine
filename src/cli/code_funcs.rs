use std::collections::HashMap;

use crate::utils;

pub fn main(code: &mut HashMap<u8, String>, params: Vec<&str>) {

    if params.len() == 0 {
        code_display(code);
    } else if params[0] == "write" {
        code_write(code, params);
    } else {
        println!("Couln't find that command");
    }


}

pub fn code_display(code: &mut HashMap<u8, String>) {
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = code.get(&x).unwrap_or(String::from("---"));
        println!("{}: {}", address, val);
    }
}

pub fn code_write(code: &mut HashMap<u8, String>, params: Vec<&str>) {
    
    if params.len() != 3 {
        utils::generic_error("Wrong amount of params");
        return;
    } else if !!!utils::is_u8(params[1]) {
        utils::generic_error("Address isn't in u8");
        return;
    }

    code.insert(
        utils::str_to_u8(params[1]),
        String::from(params[2])
    );
    println!("Wrote value");

    return;
}
