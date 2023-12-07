use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_values(line: &str) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    let part: &str = line.split(":").last().unwrap();
    let mut digits = Vec::new();

    for c in part.chars() {
        if c.is_digit(10) {
            digits.push(c);
        }
    }

    if digits.len() > 0 {
        let number = digits.into_iter().collect::<String>().parse::<usize>().unwrap();
        values.push(number);
    }

    values
}

fn calc_wins(time: usize, record: usize) -> usize {
    let mut wins = 0;
    static mut race: i32 = 0;
    unsafe {
        println!("race:{} time:{} record:{}", race, time, record);
        race += 1;
    }
    for i in 1..time {
        let travel_time = time - i;
        let distance = i * travel_time;
        if distance > record {
            wins += 1
        }
    }

    println!("wins:{}", wins);

    wins
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
    let times: Vec<usize> = get_values(lines[0]);
    let records: Vec<usize> = get_values(lines[1]);

    let races: Vec<(usize, usize)> = times.into_iter().zip(records.into_iter()).collect();

    let combinations: usize = races.into_iter().map(|(time, record)| calc_wins(time, record)).product();
    println!("combinations:{}", combinations);
}
