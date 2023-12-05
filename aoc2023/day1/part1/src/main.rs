use std::{fs::File, io::Read};
fn main() {
    let mut result: u32 = 0;
    for line in input_text().lines() {
        let line = line.trim();
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        for char in line.chars() {
            if char.is_numeric() {
                if first == None {
                    first = Some(char);
                }
                last = Some(char)
            }
        }
        let first: u32 = first.unwrap().to_digit(10).unwrap();
        let last: u32 = last.unwrap().to_digit(10).unwrap();
        result = result + (first * 10 + last);
    }
    println!("The result is: '{}'", result)
}

fn input_text() -> String {
    let mut input_text = match File::open("input.txt") {
        Ok(file) => file,
        Err(_) => panic!("Error reading File")
    };
    let mut input_buf: String = String::new();
    input_text.read_to_string(&mut input_buf).unwrap();
    input_buf
}