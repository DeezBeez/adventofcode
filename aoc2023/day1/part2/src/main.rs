use std::{fs::File, io::Read};

enum Error {
    CONVERSION(String)
}

#[allow(unused_assignments)]
fn main() {
    let mut result: u32 = 0;
    for line in input_text().lines() {
        let line = line.trim();
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        let chars: Vec<char> = line.chars().collect();
        for mut i in 0..chars.len() {
            if chars[i].is_numeric() {
                if first.is_none() {
                    first = Some(chars[i]);
                }
                last = Some(chars[i]);
            }
            else {
                for j in i..chars.len() {
                    let num_text: Vec<char> = chars.iter().cloned().skip(i).take(j - i + 1).collect();
                    let num_text: String = num_text.iter().collect();
                    if let Ok(num) = match_num(num_text) {
                        if first.is_none() {
                            first = Some(num as char);
                        }
                        last = Some(num as char);
                        i = j;
                        break;
                    }
                }
            }
        }
        let first: u32 = first.unwrap().to_digit(10).unwrap();
        let last: u32 = last.unwrap().to_digit(10).unwrap();
        result += first * 10 + last;
    }
    println!("{}", result);
}

fn match_num(num_text: String) -> Result<char, Error> {
    match num_text.as_str() {
        "one" => Ok('1'),
        "two" => Ok('2'),
        "three" => Ok('3'),
        "four" => Ok('4'),
        "five" => Ok('5'),
        "six" => Ok('6'),
        "seven" => Ok('7'),
        "eight" => Ok('8'),
        "nine" => Ok('9'),
        "zero" => Ok('0'),
        text => Err(Error::CONVERSION(format!("Error converting '{}'!", text)))
    }
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