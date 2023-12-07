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

fn parse_schematic_for_stars(schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut stars = Vec::new();

    for (row,r) in schematic.iter().enumerate() {
        for (col, c) in r.iter().enumerate() {
            if *c == '*' {
                stars.push((row, col));
            }
        }
    }

    stars
}

fn parse_schematic_for_part_numbers(schematic: &Vec<Vec<char>>) -> HashMap<(usize,usize), i32> {
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
                    for i in 0..count {
                        part_numbers.push(((row, col+i), part_number));
                    }
                    partno_chars = Vec::new();
                }
            }
        }
    }

    part_numbers.into_iter().collect()
}

fn check_up(row: usize, col: usize, parts: &HashMap<(usize,usize), i32>) -> Option<i32> {
    let y = match row.checked_sub(1) {
        Some(y) => y,
        None => return None,
    };

    match parts.get(&(y, col)) {
        Some(p) => Some(*p),
        None => None,
    }
}

fn check_down(row: usize, col: usize, schematic: &Vec<Vec<char>>, parts: &HashMap<(usize,usize), i32>) -> Option<i32> {
    let y = row + 1;
    if y > schematic.len() {
        return None;
    }

    match parts.get(&(y, col)) {
        Some(p) => Some(*p),
        None => None,
    }
}

fn adjacent_gears(star: (usize, usize), schematic: &Vec<Vec<char>>, parts: &HashMap<(usize, usize), i32>) -> i32 {
    let mut gears: HashMap<i32, i32> = HashMap::new();

    let (row, col) = star;

    //up
    match check_up(row, col, parts) {
        Some(p) => {
            gears.insert(p, 1);
        },
        None => (),
    }

    //down
    match check_down(row, col, schematic,  parts) {
        Some(p) => {
            gears.insert(p, 1);
        },
        None => (),
    }

    if col > 0 {
        //up left
        match check_up(row, col - 1, parts) {
            Some(p) => {
                gears.insert(p, 1);
            },
            None => (),
        }

        //left
        match parts.get(&(row, col - 1)) {
            Some(&p) => {
                gears.insert(p, 1);
            },
            None => (),
        }

        //down left
        match check_down(row, col - 1, schematic,  parts) {
            Some(p) => {
                gears.insert(p, 1);
            },
            None => (),
        }
    }

    if col + 1 < schematic[0].len() {
        //up right
        match check_up(row, col + 1, parts) {
            Some(p) => {
                gears.insert(p, 1);
            },
            None => (),
        }

        //right
        match parts.get(&(row, col + 1)) {
            Some(&p) => {
                gears.insert(p, 1);
            },
            None => (),
        }

        //down rigth
        match check_down(row, col + 1, schematic,  parts) {
            Some(p) => {
                gears.insert(p, 1);
            },
            None => (),
        }
    }

    if gears.len() < 2 {
        return 0;
    } else if gears.len() > 2 {
        panic!("too many gears!");
    }

    println!("gears:{:?}", gears);

    gears.into_keys().product()
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
    let stars = parse_schematic_for_stars(&schematic);
    //println!("part_numbers:{:?}", part_numbers);

    //let sum: i32 = part_numbers.into_iter().filter(|o| is_valid_part_number(o, &schematic)).inspect(|o| println!("valid part:{:?}", o)).map(|o| o.number).sum();
    let sum: i32 = stars.into_iter().map(|s| adjacent_gears(s, &schematic, &part_numbers)).sum();
    println!("Part number sum:{}", sum);
}
