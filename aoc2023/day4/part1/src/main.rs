use std::fs::File;
use std::io::Read;

fn main() {
    println!("{}",process(input_text()));
}

fn process(text: String) -> String {
    let mut result = 0;

    let mut cards: Vec<String> = Vec::new();
    for line in text.lines() {
        let a: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
        cards.push(a[1].clone());
    }

    for card in cards {
        let split_cards: Vec<String> = card.split('|').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        let winning_cards: Vec<String> = split_cards[0].trim().split(' ').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        let own_cards: Vec<String> = split_cards[1].trim().split(' ').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        let mut winners = 0;
        println!("Card: {}", card);
        'a: for card in &own_cards {
            for winning_card in &winning_cards {
                if card == winning_card {
                    winners = winners + 1;
                    println!("{} equals {}, Winners + 1", card, winning_card);
                    continue 'a;
                }
            }
        }
        if winners > 0 {
            let mut points = 0;
            for i in 0..winners {
                if i == 0 {
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
            println!("Winners: {}\n -> Points: {}", winners, points);
            result = result + points;
            println!("Result is now {}\n", result)
        }
    }

    result.to_string()
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
