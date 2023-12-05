use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Grab<'a> {
    cubes: Vec<(i32, &'a str)>
}

impl<'a> Grab<'a> {
    fn new(line: &str) -> Grab {
        let parts: Vec<&str> = line.split(", ").collect();
        let count_colour_str = parts.into_iter().map(|o| o.split(" ").collect::<Vec<&str>>());
        let cubes = count_colour_str.map(|o| (o[0].parse::<i32>().unwrap(), o[1])).collect();
        Grab { cubes: cubes }
    }
}

fn id(line: &str) -> i32 {
   line.split(" ").last().unwrap().parse::<i32>().unwrap()
}

fn grabs(line: &str) -> Vec<Grab> {
    let parts: Vec<&str> = line.split("; ").collect();
    parts.into_iter().map(|o| Grab::new(o)).collect()
}

fn line_to_game(line: &str) -> (i32, Vec<Grab>) {
    let parts: Vec<&str> = line.split(": ").collect();
    (id(parts[0]), grabs(parts[1]))
}

fn power(game: &(i32, Vec<Grab>)) -> (i32, i32) {
    let id = game.0;
    let mut map: HashMap<&str, i32> = HashMap::new();

    for grab in game.1.iter() {
        for cubes in grab.cubes.iter() {
            let count = cubes.0;
            let colour = cubes.1;
            map.entry(colour).and_modify(|o| if *o < count { *o = count }).or_insert(count);
        }
    }

    //println!("id:{} map:{:?}", id, map);
    let power = map.into_values().product();

    (id, power)
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

    //let games: Vec<(i32, Vec<Grab>)> = lines.into_iter().map(|o| line_to_game(o)).collect();
    let powers: Vec<(i32, i32)> = lines.into_iter().map(|o| line_to_game(o)).map(|o| power(&o)).collect();
    //println!("games:{:?}", games);
    let power: i32 = powers.into_iter().map(|o| o.1).sum();
    println!("power:{}", power);

}
