use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

fn line_to_history(line: &str) -> Vec<i64> {
    line.split(" ").map(|o| o.parse::<i64>().unwrap()).collect()
}

fn difference_end(values: &Vec<i64>) -> bool {
    values.iter().filter(|o| **o == 0).count() == values.len()
}

fn calc_difference(values: &Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::new();
    for i in 0..(values.len() - 1) {
        let d = values[i+1] - values[i];
        diff.push(d);
    }

    println!("diff:{:?}", diff);
    diff
}

fn predict_at(working: &Vec<Vec<i64>>, i: usize) -> i64 {
    let cur_last = working[i].iter().last().unwrap();
    let prev_last = working[i-1].iter().last().unwrap();

    prev_last + cur_last
}

fn predict(history: Vec<i64>) -> i64 {
    let mut working: Vec<Vec<i64>> = Vec::new();
    let mut index = 0;
    working.push(history.clone());

    while !difference_end(&working[index]) {
        let difference = calc_difference(&working[index]);
        working.push(difference);
        index += 1;
    }

    for i in (1..index+1).rev() {
        //println!("i:{} working:{:?}", i, working[i]);
        let prediction = predict_at(&working, i);
        working[i-1].push(prediction);
    }

    *working[0].iter().last().unwrap()
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
    let historys: Vec<Vec<i64>> = lines.into_iter().map(|o| line_to_history(o)).collect();
    //println!("historys:{:?}", historys);
    let predictions: Vec<i64> = historys.into_iter().map(|o| predict(o)).collect();
    let sum: i64 = predictions.into_iter().sum();
    println!("predictions sum:{}", sum);
}
