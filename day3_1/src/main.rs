use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct PartNo {
    number: i32,
    row: usize,
    col: usize,
    count: usize,

}

impl PartNo {
    fn new(number: i32, row: usize, col: usize, count: usize) -> PartNo {
        PartNo {number, row, col, count}
    }
}

fn parse_schematic_for_part_numbers(schematic: &Vec<Vec<char>>) -> Vec<PartNo> {
    let mut part_numbers = Vec::new();
    let mut partno_chars = Vec::new();
    let mut row = 0;
    let mut col = 0;

    for (x,r) in schematic.iter().enumerate() {
        for (y, c) in r.iter().enumerate() {
            if c.is_digit(10) {
                if partno_chars.len() == 0 {
                    row = x;
                    col = y;
                }
                partno_chars.push(c);
            } else {
                if partno_chars.len() > 0 {
                    let count = partno_chars.len();
                    let part_number = partno_chars.into_iter().collect::<String>().parse::<i32>().unwrap();
                    part_numbers.push(PartNo::new(part_number, row, col, count));
                    partno_chars = Vec::new();
                }
            }
        }
    }

    part_numbers
}

fn is_symbol(c: char) -> bool {
    if c != '.' && !c.is_digit(10) {
        true
    } else {
        false
    }
}

fn check_up(row: usize, col: usize, schematic: &Vec<Vec<char>>) -> bool {
    match row.checked_sub(1) {
        Some(r) => check(r, col, schematic),
        None => false,
    }
}

fn check_down(row: usize, col: usize, schematic: &Vec<Vec<char>>) -> bool {
    if row + 1 < schematic.len() {
        check(row + 1, col, schematic)
    } else {
        false
    }

}

fn check_left(row: usize, col: usize, schematic: &Vec<Vec<char>>) -> bool {
    if col > 0 {
        check(row, col - 1, schematic)
    } else {
        false
    }

}

fn check_right(row: usize, col: usize, schematic: &Vec<Vec<char>>) -> bool {
    let x = col + 1;
    if x < schematic[0].len() {
        check(row, x, schematic)
    } else {
        false
    }

}

fn check_diag(row: usize, col: usize, schematic: &Vec<Vec<char>>) -> bool {
    if col > 0 {
        let x = col - 1;
        if check_up(row, x, schematic) || check_down(row, x, schematic) {
            return true;
        }
    }

    if col + 1 < schematic[0].len() {
        let x = col + 1;
        if check_up(row, x, schematic) || check_down(row, x, schematic) {
            return true;
        }
    }
    false
}

fn check(row: usize, col: usize, schematic: &Vec<Vec<char>>) -> bool {
    if is_symbol(schematic[row][col]) {
        true
    } else {
        false
    }
}

fn check_angles(row: usize, col: usize,  partno: &PartNo, schematic: &Vec<Vec<char>>) -> bool {
    if row > schematic.len() {
        panic!("Row out of bounds");
    }
    if col > schematic[0].len() {
        panic!("Col out of bounds");
    }

    if check_up(row, col, schematic) 
    || check_down(row, col, schematic)
    || check_left(row, col, schematic)
    || check_right(row, col, schematic)
    || check_diag(row, col, schematic)
    {
        true
    } else {
        false
    }
}

fn is_valid_part_number(partno: &PartNo, schematic: &Vec<Vec<char>>) -> bool {

    for i in 0..partno.count {
        if check_angles(partno.row, partno.col + i, partno, schematic) {
            return true;
        }
    }
    false
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

    let schematic: Vec<Vec<char>> = lines.into_iter().map(|o| o.chars().collect()).collect();
    //println!("schematic:{:?}", schematic);
    let part_numbers = parse_schematic_for_part_numbers(&schematic);
    //println!("part_numbers:{:?}", part_numbers);

    let sum: i32 = part_numbers.into_iter().filter(|o| is_valid_part_number(o, &schematic)).inspect(|o| println!("valid part:{:?}", o)).map(|o| o.number).sum();
    println!("Part number sum:{}", sum);
}
