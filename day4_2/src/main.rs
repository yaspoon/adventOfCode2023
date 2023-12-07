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

fn parse_line_for_card(line: &str) -> (usize, Card) {
    let parts: Vec<&str> = line.split(": ").collect();
    let id = get_card_id(parts[0]);
    let (winning_numbers, numbers) = parse_numbers(parts[1]);
    (id, Card::new(id, winning_numbers, numbers))
}

fn is_winning(n: &usize, winning_numbers: &HashMap<usize, bool>) -> bool {
    if winning_numbers.contains_key(n) {
        true
    } else {
        false
    }
}

fn card_wins(card: &Card) -> usize {
    card.numbers.iter().filter(|n| is_winning(n, &card.winning_numbers)).count()
}

/*
fn get_counts_for_card(card: usize, cards: &HashMap<usize, Card>, lookup: &mut HashMap<usize, HashMap<usize, usize>>) -> HashMap<usize,usize> {
    let mut card_counts: HashMap<usize, usize> = HashMap::new();
    card_counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
    if card <= cards.len() {
        if lookup.contains_key(&card) {
            for (id, number) in lookup.get(&card).unwrap() {
                card_counts.insert(*id, *number);
            }
        } else {
            let wins = card_points(cards.get(&card).unwrap());
            println!("card:{} wins:{}", card, wins);
            let start = card + 1;
            let end = start + wins;
            for i in start..end {
                if i < cards.len() {
                    card_counts.entry(i).and_modify(|c| *c += 1).or_insert(1);
                    println!("card:{} start:{} end:{}", card, start, end);
                    let sub_count = get_counts_for_card(i, cards, lookup);
                    println!("sub_count:{}", sub_count.len());
                    if sub_count.len() > 0 {
                        for (id, count) in sub_count.iter() {
                            card_counts.entry(*id).and_modify(|c| *c += count).or_insert(*count);
                        }
                    }
                }
            }
        }
    }

    card_counts
}
*/

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
    let input: Vec<(usize, Card)> = lines.into_iter().map(|l| parse_line_for_card(l)).collect();
    let max_card = input.len();
    let mut todo: Vec<usize> = input.iter().map(|o| o.0).collect();
    let cards: HashMap<usize, Card> = input.into_iter().collect();
    /*
    let mut lookup: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    */
    println!("max_card:{}", max_card);
    println!("todo:{:?}", todo);

    let mut count = 0;
    while todo.len() > 0 {
        let cur = todo.pop().unwrap();
        count += 1;
        let wins = card_wins(cards.get(&cur).unwrap());
        let start = cur + 1;
        let end = start + wins;
        for i in start..end {
            if i <= max_card {
                todo.push(i);
            }
        }
        /*
        let counts_update = get_counts_for_card(cur, &cards, &mut lookup);
        for (id, count) in counts_update.iter() {
            counts.entry(*id).and_modify(|c| *c += count).or_insert(*count);
        }
        */
    }
    println!("total_count:{}", count);

}
