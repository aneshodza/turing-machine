use std::str::SplitWhitespace;

pub fn get_hex_string(num: u8) -> String {
    if num > 16 {
        return String::from(format!("0x{:X}", num));
    } else {
        return String::from(format!("0x0{:X}", num));
    }
}

pub fn parse_input_to_cmd(input: &SplitWhitespace) -> String {
    return String::from(input.clone().next().unwrap_or(":h"));
}

pub fn parse_input_to_vec<'a>(input: SplitWhitespace<'a>) -> Vec<&'a str> {
    return input.clone().collect();
}

pub fn str_to_u8(input: &str) -> u8 {
    return input.to_string().parse::<u8>().unwrap();
}

pub fn is_u8(input: &str) -> bool {
    return input.parse::<u8>().is_ok();
}

pub fn generic_error(msg: &str) {
    println!("Can't handle this -> {}", msg);
}

pub fn join_vec_to_string(v: Vec<&str>, join: String) -> String {
    let mut ret_string: String = "".to_string();

    for i in v[2..].iter() {
        ret_string += &(join.clone() + i);
    }
    ret_string.remove(0);

    return ret_string;
}
