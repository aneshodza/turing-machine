use std::collections::HashMap;

use crate::utils;

pub fn main(code: &mut HashMap<u8, String>, params: Vec<&str>) {

    if params.len() == 0 {
        code_display(code);
    } else if params[0] == "write" {
        code_write(code, params);
    } else if params[0] == "clear" {
        code_clear(code, params);
    }
    else {
        println!("Couln't find that command");
    }


}

fn code_display(code: &mut HashMap<u8, String>) {
    let empty_val = String::from("---");
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = code.get(&x).unwrap_or(&empty_val);
        println!("{}: {}", address, val);
    }
}

fn code_write(code: &mut HashMap<u8, String>, params: Vec<&str>) {    
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
    if params[2] != "---" {
        println!("Wrote value");
    } else {
        println!("Cleared value");
    }

    return;
}

fn code_clear(code: &mut HashMap<u8, String>, params: Vec<&str>) {
    if params.len() != 2 {
        utils::generic_error("Wrong amount of params");
        return;
    }
    code_write(code, vec![params[0], params[1], "---"]);
    return;
}
