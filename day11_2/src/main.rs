use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

fn rotate_universe(universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated: Vec<Vec<char>> = Vec::new();

    for _ in 0..universe[0].len() {
        rotated.push(Vec::new());
    }

    for row in universe.iter() {
        for (i,c) in row.iter().enumerate() {
            rotated[i].push(*c);
        }
    }

    rotated
}

fn expand_universe(lines: Vec<&str>) -> Vec<Vec<char>> {
    let universe: Vec<Vec<char>> = lines.into_iter().map(|o| o.chars().collect()).collect();
    //println!("universe rows:{} cols:{}", universe.len(), universe[0].len());
    //print_universe(&universe);
    let mut row_expanded: Vec<Vec<char>> = Vec::new();

    for line in universe {
        row_expanded.push(line.clone());
        if line.iter().filter(|o| **o == '.').count() == line.len() {
            for _ in 0..(1000000-1) {
                row_expanded.push(line.clone());
            }
        }
    }

    let rotated = rotate_universe(row_expanded);

    let mut col_expanded: Vec<Vec<char>> = Vec::new();
    for line in rotated {
        col_expanded.push(line.clone());
        if line.iter().filter(|o| **o == '.').count() == line.len() {
            for _ in 0..(1000000-1) {
                col_expanded.push(line.clone());
            }
        }
    }

    let expanded_universe = rotate_universe(col_expanded);

    expanded_universe
}

fn print_universe(universe: &Vec<Vec<char>>) {
    for l in universe.iter() {
        for c in l.iter() {
            print!("{} ", c);
        }
        println!("");
    }
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<(isize,isize)> {
    let mut galaxies: Vec<(isize, isize)> = Vec::new();

    for (y,row) in universe.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                galaxies.push((x as isize, y as isize));
            }
        }
    }

    galaxies
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

    let expanded = expand_universe(lines);
    //print_universe(&expanded);
    //println!("expanded rows:{} cols:{}", expanded.len(), expanded[0].len());

    let galaxies: Vec<(isize,isize)> = find_galaxies(&expanded);
    let gl = galaxies.len();
    //println!("gl:{}", gl);

    let mut done: HashSet<usize> = HashSet::new();

    let mut distances: Vec<isize> = Vec::new();
    for (i, (p1, p2)) in galaxies.iter().enumerate() {
        for (j, (q1, q2)) in galaxies.iter().enumerate() {
            if i != j {
                if !done.contains(&j) {
                    let distance = (p1 - q1).abs() + (p2 - q2).abs();
                    distances.push(distance);
                }
            }
        }
        done.insert(i);
    }

    println!("distances:{}", distances.len());

    let sum: isize = distances.into_iter().sum();
    println!("sum:{}", sum);
}
