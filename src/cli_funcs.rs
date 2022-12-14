use std::collections::HashMap;

use crate::utils;

pub fn list_storage(memory_ref: &HashMap<String, u8>) {
    for x in 0..16 {
        let address = utils::get_hex_string(x);
        let val = memory_ref.get(&address).unwrap_or(&0);
        println!("{}: {}", address, utils::get_hex_string(*val))
    }
}
