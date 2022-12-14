pub fn get_hex_string(num: u8) -> String {
    if num > 16 {
        return String::from(format!("0x{:X}", num));
    } else {
        return String::from(format!("0x0{:X}", num));
    }
}

pub fn str_to_u8(input: &str) -> u8 {
    return input.to_string().parse::<u8>().unwrap();
}
