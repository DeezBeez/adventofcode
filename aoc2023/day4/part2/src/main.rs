use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Card {
    index: usize,
    winners: usize,
    count: usize
}

impl Card {
    fn new(index: usize, winners: usize) -> Self {
        Self { index, winners, count: 1 }
    }
}

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

    let mut all_cards: Vec<Card> = Vec::new();
    for i in 0..cards.len() {
        all_cards.push(Card::new(i, 0))
    }

    for card in &mut all_cards {
        let winners = get_winners(&cards[card.index]);
        card.winners = winners;
        
    }

    for i in 0..all_cards.len() {
        for j in 0..all_cards[i].winners {
            all_cards[i+j+1].count = all_cards[i+j+1].count + (1 * all_cards[i].count);
        }
    }

    for card in &all_cards {
        result = result + card.count;
    }

    println!("Card: {:?}", all_cards);

    result.to_string()
}

fn get_winners(card: &String) -> usize{
    let split_cards: Vec<String> = card.split('|').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
    let winning_cards: Vec<String> = split_cards[0].trim().split(' ').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
    let own_cards: Vec<String> = split_cards[1].trim().split(' ').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
    let mut winners = 0;
    'a: for card in &own_cards {
        for winning_card in &winning_cards {
            if card == winning_card {
                winners = winners + 1;
                continue 'a;
            }
        }
    }
    winners
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
