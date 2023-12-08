use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

fn create_node(line: &str) -> (&str, (&str, &str)) {
    let parts: Vec<&str> = line.split(" = ").collect();
    let name = parts[0];
    let stripped = parts[1].strip_prefix("(").unwrap().strip_suffix(")").unwrap();
    //println!("stripped:{}", stripped);
    let nodes: Vec<&str> = stripped.split(", ").collect();
    let left = nodes[0];
    let right = nodes[1];
    (name, (left, right))
}

fn get_starting_nodes<'a>(nodes: &HashMap<&'a str, (&str, &str)>) -> Vec<&'a str> {
    nodes.keys().filter(|n| n.chars().last().unwrap() == 'A').map(|n| *n).collect()
}

fn check_end_condition(nodes: &Vec<&str>) -> bool {
    let total = nodes.len();
    let ending_in_z = nodes.iter().filter(|n| n.chars().last().unwrap() == 'Z').count();

    if total == ending_in_z {
        true
    } else {
        false
    }
}

fn main() {
    let path = Path::new("sample_input");
    //let path = Path::new("input");

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

    let mut cur_state: Vec<&str> = get_starting_nodes(&nodes);
    println!("cur_state:{:?}", cur_state);
    let mut instruction_index = 0;
    let mut steps = 0;
    let mut done_steps: Vec<usize> = Vec::new();
    for _ in 0..cur_state.len() {
        done_steps.push(0);
    }

    while !check_end_condition(&cur_state) {
        steps += 1;
        println!("step:{}", steps);
        let instruction = instructions.get(instruction_index).unwrap();
        instruction_index += 1;
        if instruction_index >= instructions.len() {
            instruction_index = 0;
        }

        for cur_index in 0..cur_state.len() {
            let cur = cur_state[cur_index];
            let (left, right) = match nodes.get(cur) {
                Some(n) => n,
                None => panic!("No such node:{}", cur),
            };

            if cur.chars().last().unwrap() != 'Z' {
                match instruction {
                    'L' => {
                        cur_state[cur_index] = left;
                        //println!("left:{}", cur_state[cur_index]);
                    },
                    'R' => {
                        cur_state[cur_index] = right;
                        //println!("right:{}", cur_state[cur_index]);
                    },
                    _ => panic!("Unknown instruction:{}", instruction),
                }
            }

            if cur.chars().last().unwrap() == 'Z' {
                if done_steps[cur_index] == 0 {
                    done_steps[cur_index] = steps;
                }
            }
        }
    }

    //println!("Steps:{}", steps);
    println!("done_steps:{:?}", done_steps);
}
