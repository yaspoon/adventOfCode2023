use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

fn create_map(lines: Vec<&str>) -> Vec<Vec<char>> {
    lines.into_iter().map(|o| o.chars().collect()).collect()
}

fn find_start(map: &Vec<Vec<char>>) -> (usize,usize) {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                return (y,x);
            }
        }
    }

    panic!("Failed to find start");
}

fn print_map(map: &Vec<Vec<char>>) {
    for l in map.iter() {
        for c in l.iter() {
            print!("{}", c);
        }
        println!("");
    }
}

fn is_valid(x: usize, y: usize, map: &Vec<Vec<char>>) -> bool {
    if map[y][x] == '.' || map[y][x] == 'S' {
        true
    } else {
        false
    }
}

fn modify_map(paths: &HashSet<(usize, usize)>, map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for l in map.iter() {
        new_map.push(l.clone());
    }

    for (y, x) in paths.iter() {
        new_map[*y][*x] = 'O';
    }

    new_map
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

    let map = create_map(lines);
    let height = map.len();
    let width = map[0].len();
    print_map(&map);
    let start = find_start(&map);
    println!("start: x:{} y:{}", start.1, start.0);

    let mut paths: HashSet<(usize,usize)> = HashSet::new();
    paths.insert(start);

    const STEP_COUNT: usize = 64;
    for _ in 0..STEP_COUNT {
        let mut new_paths: HashSet<(usize, usize)> = HashSet::new();
        for (y, x) in paths.iter() {
            //north
            if *y > 0 {
                let new_y = *y - 1;
                if is_valid(*x, new_y, &map) && !new_paths.contains(&(new_y, *x)) {
                    new_paths.insert((new_y, *x));
                }
            }

            //south
            if (*y + 1) < height {
                let new_y = *y + 1;
                if is_valid(*x, new_y, &map) && !new_paths.contains(&(new_y, *x)) {
                    new_paths.insert((new_y, *x));
                }
            }

            //West
            if *x > 0 {
                let new_x = *x - 1;
                if is_valid(new_x, *y, &map) && !new_paths.contains(&(*y, new_x)) {
                    new_paths.insert((*y, new_x));
                }
            }

            //East
            if (*x + 1) < width {
                let new_x = *x + 1;
                if is_valid(new_x, *y, &map) && !new_paths.contains(&(*y, new_x)) {
                    new_paths.insert((*y, new_x));
                }
            }
        }

        /*
        for np in new_paths {
            paths.insert(np);
        }
        */
        paths = new_paths;
    }

    let plot_maps = modify_map(&paths, &map);
    print_map(&plot_maps);

    println!("Reachable garden plots:{}", paths.len());
}
