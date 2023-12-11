use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

fn create_hand(line: &str) -> (Vec<char>, usize) {
    let mut cards: Vec<char>;
    let mut bid: usize;
    let parts: Vec<&str> = line.split(" ").collect();
    if let [hand, num] = parts[..] {
        cards = hand.chars().collect();
        bid = num.parse::<usize>().unwrap();
    } else {
        panic!("Failed to parse hand");
    }
    (cards, bid)
}

#[derive(Clone)]
enum Strength {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


fn get_hand_strength(hand: &(Vec<char>, usize)) -> Strength {
    let mut card_map: HashMap<char,usize> = HashMap::new();
    hand.0.iter().for_each(|card| {
        card_map.entry(*card).and_modify(|c| *c += 1).or_insert(1);
    });
    let mut counts_map: HashMap<usize, usize> = HashMap::new();
    card_map.values().for_each(|v| {
        counts_map.entry(*v).and_modify(|c| *c += 1).or_insert(1);
    });

    let has_joker = card_map.contains_key(&'J');
    let joker_count = match card_map.get(&'J') {
        Some(n) => *n,
        None => 0,
    };
    let max_count = counts_map.keys().copied().max().unwrap();

    if card_map.len() == 1 {
        Strength::FiveOfAKind
    } else if card_map.len() == 2 {
        if has_joker {
            Strength::FiveOfAKind
        } else {
            if counts_map.contains_key(&4) {
                Strength::FourOfAKind
            } else {
                Strength::FullHouse
            }
        }
    } else if card_map.len() == 3 {
        if has_joker {
            if joker_count == 1 {
                if max_count == 3 {
                    Strength::FourOfAKind
                } else {
                    Strength::FullHouse
                }
            } else {
                Strength::FourOfAKind
            }
        } else {
            if counts_map.contains_key(&3) {
                Strength::ThreeOfAKind
            } else {
                Strength::TwoPair
            }
        }
    } else if card_map.len() == 4 {
        if has_joker {
            Strength::ThreeOfAKind
        } else {
            Strength::OnePair
        }
    } else {
        if has_joker {
            Strength::OnePair
        } else {
            Strength::HighCard
        }
    }
}

fn map_card_to_strength(card: char) -> isize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("Unknown card: {}", card),
    }
}

fn secondary_strength_test(left: &(Vec<char>, usize), right: &(Vec<char>, usize)) -> Ordering {
    let pairs: Vec<(char, char)> = left.0.iter().copied().zip(right.0.iter().copied()).collect();

    for (left_card, right_card) in pairs {
        if left_card == right_card {
            continue;
        } else {
            let left_stren = map_card_to_strength(left_card);
            let right_stren = map_card_to_strength(right_card);
            if left_stren > right_stren {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
    }
    Ordering::Equal
}

fn compare_strength(left: &(Vec<char>, usize), right: &(Vec<char>, usize)) -> Ordering {
    let stren_left = get_hand_strength(left);
    let stren_right = get_hand_strength(right);

    if stren_left.clone() as isize == stren_right.clone() as isize {
        secondary_strength_test(left, right)
    } else if stren_left as isize > stren_right as isize {
        Ordering::Greater
    } else {
        Ordering::Less
    }
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
    let count = lines.len();
    println!("count:{}", count);
    let mut ranks = Vec::new();
    for r in 1..(count+1) {
        ranks.push(r);
    }
    println!("ranks:{:?}", ranks);
    let mut hands: Vec<(Vec<char>, usize)> = lines.into_iter().map(|o| create_hand(o)).collect();
    hands.sort_by(|a, b| compare_strength(a, b));
    println!("hands:{:?}", hands);
    let total_winnings: usize = hands.into_iter().zip(ranks.into_iter()).map(|(hand,rank)| hand.1 * rank).sum();
    println!("total_winnings:{}", total_winnings);

}
