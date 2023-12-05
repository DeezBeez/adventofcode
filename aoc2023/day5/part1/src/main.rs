use std::fs::File;
use std::io::Read;

struct Map {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl Map {
    fn new(destination_range_start: usize, source_range_start: usize, range_length: usize) -> Self {
        Self { destination_range_start, source_range_start, range_length }
    }
}

fn main() {
    println!("{}",process(input_text()));
}

fn process(text: String) -> String {
    let mut result = 0;
    let mut seeds: Vec<String> = Vec::new();
    for line in text.lines() {
        let split: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
        if split.len() == 2 && split[0] == "seeds" {
            seeds = split[1].trim().split(' ').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
            break;
        }
    }
    
    result.to_string()
}

fn seed_to_soil(seed: String) -> Vec<Map> {
    Vec::new()
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
