use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashMap<usize, bool>,
    numbers: Vec<usize>,
}

impl Card {
    fn new(id: usize, winning_numbers: HashMap<usize, bool>, numbers: Vec<usize>) -> Card {
        Card {id, winning_numbers, numbers}
    }
}

fn get_card_id(line: &str) -> usize {
    line.split(" ").last().unwrap().parse::<usize>().unwrap()
}

fn get_winning_numbers(line: &str) -> HashMap<usize, bool> {
    line.split(" ").filter(|o| *o != "").map(|o| (o.parse::<usize>().unwrap(), true)).collect()
}

fn get_numbers(line: &str) -> Vec<usize> {
    line.split(" ").filter(|o| *o != "").map(|o| o.parse::<usize>().unwrap()).collect()
}

fn parse_numbers(line: &str) -> (HashMap<usize, bool>, Vec<usize>) {
    let parts: Vec<&str> = line.split(" | ").collect();
    let winning_numbers = get_winning_numbers(parts[0]);
    let numbers = get_numbers(parts[1]);

    (winning_numbers, numbers)
}

fn parse_line_for_card(line: &str) -> Card {
    let parts: Vec<&str> = line.split(": ").collect();
    let id = get_card_id(parts[0]);
    let (winning_numbers, numbers) = parse_numbers(parts[1]);
    Card::new(id, winning_numbers, numbers)
}

fn is_winning(n: &usize, winning_numbers: &HashMap<usize, bool>) -> bool {
    if winning_numbers.contains_key(n) {
        true
    } else {
        false
    }
}

fn card_points(card: Card) -> usize {
    let winning_number_count: u32 = card.numbers.into_iter().filter(|n| is_winning(n, &card.winning_numbers)).count() as u32;
    let mut points = 0;
    if winning_number_count > 0 {
        let power = 2usize.pow(winning_number_count-1);
        points = 1 + (power - 1);
    }
    println!("card:{} points:{}", card.id, points);
    points
}

fn main() {
    //let path = Path::new("sample_input");
    let path = Path::new("input");

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file:{:?}", path),
    };

    let mut input: String = String::new();
    match file.read_to_string(&mut input) {
        Ok(_) => (),
        Err(e) => panic!("Failed to read file into string"),
    }

    let lines: Vec<&str> = input.lines().collect();
    let cards: Vec<Card> = lines.into_iter().map(|l| parse_line_for_card(l)).collect();
    //println!("cards:{:?}", cards);
    let points: usize = cards.into_iter().map(|c| card_points(c)).sum();
    println!("points:{}", points);

}
