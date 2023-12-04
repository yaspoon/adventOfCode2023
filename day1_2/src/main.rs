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
    let names = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut digits: Vec<char> = Vec::new();
        loop {
            if i >= chars.len() {
                break;
            }

            let c = chars[i];
            if c.is_digit(10) {
                digits.push(c);
            } else {
                let mut found = false;
                for (num, name) in names.clone().into_iter().enumerate() {
                    if (chars.len() - i) >= name.len() {
                        if chars[i..i+name.len()] == name {
                           digits.push(char::from_digit((num + 1) as u32, 10).unwrap()); 
                           //i += name.len();
                           //found = true;
                           break;
                        }
                    }
                }
            }
            i += 1;
        }
        println!("digits:{:?}", digits);
        let asdf = vec![digits[0], digits[digits.len()-1]];
        let asdf2: String = asdf.into_iter().collect::<String>();
        let num = asdf2.parse::<i32>().unwrap();
        //println!("{}", num);
        nums.push(num);
    }

    let total: i32 = nums.into_iter().sum();

    println!("total:{}", total);

}
