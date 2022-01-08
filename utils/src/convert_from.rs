use std::str::Chars;

fn from_int_to_char(i: i32) -> char {
    std::char::from_digit(i as u32, 10).unwrap()
}

fn from_int_to_alphabet(i: i32) -> char {
    (b'a' + ((i - 1) % 26) as u8) as char
}

fn from_int_to_string(i: i32) -> String {
    i.to_string()
}

fn from_char_to_int(c: char) -> i32 {
    c.to_digit(10).unwrap() as i32
}

fn from_vec_char_to_string(cs: Vec<char>) -> String {
    cs.iter().collect::<String>()
}

fn from_chars_to_string(cs: Chars<'_>) -> String {
    cs.collect::<String>()
}

fn from_string_to_int(s: String) -> i32 {
    s.parse::<i32>().unwrap()
}

fn from_string_to_vec_char(s: String) -> Vec<char> {
    s.chars().collect::<Vec<char>>()
}
