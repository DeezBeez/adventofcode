use std::fs::File;
use std::io::Read;

fn main() {
    process(input_text());
}

fn process(text: String) {
    let lines = text.lines();
    
    let red: u32 = 12;
    let green: u32 = 13;
    let blue: u32 = 14;

    let mut game_id = 0;
    let mut result = 0;

    for line in lines {
        game_id = game_id + 1;
        let line = line.trim();
        let after_game = line.find(':').unwrap() + 1;
        let sets: Vec<String> = line[after_game..].split(';').map(|s| s.to_string()).collect();
        let mut possible = true;
        for set in sets {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            
            let counter_text: String = set.split(',').map(|s| s.to_string()).collect();
            let counter_text = counter_text.trim();
            let words: Vec<String> = counter_text.split(' ').map(|s| s.to_string()).collect();
            let mut count: u32 = 0;
            for i in 0..words.len() {
                if i == 0 { // first
                    count = words[i].parse().unwrap();
                } else if i % 2 == 0 { // is num?
                    count = words[i].parse().unwrap();
                } else { // is color
                    match words[i].as_str() {
                        "red" => r = r + count,
                        "green" => g = g + count,
                        "blue" => b = b + count,
                        _ => {}
                    }
                }
            }
            if r > red || g > green || b > blue {
                possible = false;
            }
        }
        if possible == true {
            result = result + game_id;
        }
    }

    println!("{}", result)
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