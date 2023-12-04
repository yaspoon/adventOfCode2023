use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

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

    let mut nums: Vec<i32> = Vec::new();
    for line in lines {
        let digits = line.chars().filter(|x| x.is_digit(10)).collect::<Vec<char>>();
        let asdf = vec![digits[0], digits[digits.len()-1]];
        let asdf2: String = asdf.into_iter().collect::<String>();
        let num = asdf2.parse::<i32>().unwrap();
        //println!("{}", num);
        nums.push(num);
    }

    let total: i32 = nums.into_iter().sum();

    println!("total:{}", total);

}
