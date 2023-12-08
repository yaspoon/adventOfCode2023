use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

fn create_node(line: &str) -> (&str, (&str, &str)) {
    let parts: Vec<&str> = line.split(" = ").collect();
    let name = parts[0];
    let stripped = parts[1].strip_prefix("(").unwrap().strip_suffix(")").unwrap();
    println!("stripped:{}", stripped);
    let nodes: Vec<&str> = stripped.split(", ").collect();
    let left = nodes[0];
    let right = nodes[1];
    (name, (left, right))
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
    let instructions: Vec<char> = lines[0].chars().collect();
    //println!("instructions:{:?}", instructions);

    let nodes: HashMap<&str,(&str, &str)> = lines[2..].into_iter().map(|o| create_node(o)).collect();
    //println!("nodes:{:?}", nodes);

    let mut cur = "AAA";
    let mut instruction_index = 0;
    let mut steps = 0;
    while cur != "ZZZ" {
        steps += 1;
        let (left, right) = match nodes.get(cur) {
            Some(n) => n,
            None => panic!("No such node:{}", cur),
        };
        let instruction = instructions.get(instruction_index).unwrap();
        instruction_index += 1;
        if instruction_index >= instructions.len() {
            instruction_index = 0;
        }

        match instruction {
            'L' => {
                cur = left;
            },
            'R' => {
                cur = right;
            },
            _ => panic!("Unknown instruction:{}", instruction),
        }
    }

    println!("Steps:{}", steps);
}
