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

/*
fn check_end_condition(nodes: &Vec<&str>) -> bool {
    let total = nodes.len();
    let ending_in_z = nodes.iter().filter(|n| n.chars().last().unwrap() == 'Z').count();

    if total == ending_in_z {
        true
    } else {
        false
    }
}
*/

fn greatest_common_divisor(first: usize, second: usize) -> usize {
    let (mut a, mut b) = if first > second {
        (first, second)
    } else {
        (second, first)
    };

    'end: loop {
        let r = a % b;
        if r == 0 {
            break 'end;
        } else {
            a = b;
            b = r;
        }
    }

    println!("first:{} second:{} gcd:{}", first, second, b);
    b
    
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greatest_common_divisor(a, b)
}

fn check_end_condition(done_steps: &Vec<usize>, numerator: usize) -> bool {
    let matching = done_steps.iter().filter(|n| numerator % *n == 0).count();
    matching == done_steps.len()

}

fn ends_in_z(node: &str) -> bool {
    node.chars().last().unwrap() == 'Z'
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

    let mut cur_state: Vec<&str> = get_starting_nodes(&nodes);
    println!("cur_state:{:?}", cur_state);
    //let mut instruction_index = 0;
    //let mut steps = 0;
    let mut done_steps: Vec<usize> = Vec::new();
    for i in 0..cur_state.len() {
        let mut steps = 0;
        let mut instruction_index = 0;
        let mut cur = cur_state[i];
        while !ends_in_z(cur) {
            steps += 1;
            let instruction = instructions.get(instruction_index).unwrap();
            instruction_index += 1;
            if instruction_index >= instructions.len() {
                instruction_index = 0;
            }

            let (left, right) = match nodes.get(cur) {
                Some(n) => n,
                None => panic!("No such node:{}", cur),
            };

            match instruction {
                'L' => {
                    cur = left;
                    //println!("left:{}", cur_state[cur_index]);
                },
                'R' => {
                    cur = right;
                    //println!("right:{}", cur_state[cur_index]);
                },
                _ => panic!("Unknown instruction:{}", instruction),
            }
        }

        done_steps.push(steps);
    }

    while done_steps.len() > 1 {
        println!("done_steps:{:?}", done_steps);
        done_steps = done_steps[..].chunks(2).map(|d| {
            if d.len() == 2 {
                least_common_multiple(d[0], d[1])
            } else {
                d[0]
            }
        }).collect();
    }

    /*
    let mut count = 1;
    while !check_end_condition(&done_steps, done_steps[0] * count) {
        count += 1;
    }
    */

    //println!("Steps:{}", count * done_steps[0]);
    println!("Steps:{}", done_steps[0]);
}
