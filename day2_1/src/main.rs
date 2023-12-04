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

fn is_possible(game: &(i32, Vec<Grab>)) -> bool {
    game.1.iter()
        .map(|o| o.cubes.iter()
                .map(|o| match o.1 {
                        "blue" => {
                            if o.0 > 14 {
                                false
                            } else {
                                true
                            }
                        },
                        "red" => {
                            if o.0 > 12 {
                                false
                            } else {
                                true
                            }
                        },
                        "green" => {
                            if o.0 > 13 {
                                false
                            } else {
                                true
                            }
                        }
                        _ => panic!("Not red, blue or green"),
                    }
                ).all(|o| o == true)
            ).all(|o| o == true)

    /*
    let mut map: HashMap<&str, i32> = HashMap::new();

    for grab in game.1.iter() {
        for cubes in grab.cubes.iter() {
            map.entry(cubes.1).and_modify(|c| *c += cubes.0).or_insert(cubes.0);
        }
    }

    println!("id:{} map:{:?}", game.0, map);

    map.into_iter().map(|o| match o.0 {
        "blue" => {
            if o.1 > 14 {
                false
            } else {
                true
            }
        },
        "red" => {
            if o.1 > 12 {
                false
            } else {
                true
            }
        },
        "green" => {
            if o.1 > 13 {
                false
            } else {
                true
            }
        }
        _ => panic!("Not red, blue or green"),
    }).all(|o| o == true)
    */
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
    let games: Vec<(i32, Vec<Grab>)> = lines.into_iter().map(|o| line_to_game(o)).filter(|o| is_possible(o)).collect();
    //println!("games:{:?}", games);
    let id_sum: i32 = games.into_iter().map(|o| o.0).sum();
    println!("id sum:{}", id_sum);

}
